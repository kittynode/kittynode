<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { Package } from "$lib/types";

  let packages: { [name: string]: Package } = $state({});
  let isDockerRunning: boolean | null = $state(null);
  let installedPackages: Set<string> = $state(new Set());
  let installLoading: string | null = $state(null); // Track which package is being installed
  let deleteLoading: string | null = $state(null); // Track which package is being deleted
  let ready = $state(false); // Track when state checks are complete

  async function loadPackages() {
    try {
      packages = await invoke("get_packages");
      if (isDockerRunning) {
        installedPackages = new Set(await invoke("get_installed_packages"));
      }
    } catch (error) {
      alert("Failed to load packages.");
      console.error(error);
    } finally {
      ready = true;
    }
  }

  async function installPackage(name: string) {
    installLoading = name;
    try {
      await invoke("install_package", { name });
      await loadPackages();
    } catch (error) {
      alert(`Failed to install ${name}.`);
      console.error(error);
    } finally {
      installLoading = null;
    }
  }

  async function deletePackage(name: string, includeImages: boolean) {
    deleteLoading = name;
    try {
      await invoke("delete_package", { name, includeImages });
      await loadPackages();
    } catch (error) {
      alert(`Failed to delete ${name}.`);
      console.error(error);
    } finally {
      deleteLoading = null;
    }
  }

  async function checkDocker() {
    isDockerRunning = await invoke("is_docker_running");
  }

  onMount(async () => {
    await checkDocker();
    await loadPackages();
  });
</script>

{#if Object.keys(packages).length > 0}
  {#each Object.entries(packages).sort(([a], [b]) => a.localeCompare(b)) as [name, p]}
    <article>
      <h2>{name}</h2>
      {#if !isDockerRunning}
        <p><strong>Turn on Docker to use this package. If you need to install Docker, please follow the installation guide <a href="https://docs.docker.com/engine/install/" target="_blank">here</a>.</strong></p>
      {/if}
      <p>{p.description}</p>
      <button
        onclick={() => installPackage(name)}
        disabled={!ready || !isDockerRunning || installedPackages.has(name) || installLoading === name || deleteLoading === name}
      >
        {installLoading === name ? "Installing..." : "Install"}
      </button>

      {#if installedPackages.has(name)}
        <button
          class="secondary"
          onclick={() => deletePackage(name, false)}
          disabled={!isDockerRunning || deleteLoading === name}
        >
          {deleteLoading === name ? "Deleting..." : "Delete"}
        </button>
      {/if}
    </article>
  {/each}
{/if}
