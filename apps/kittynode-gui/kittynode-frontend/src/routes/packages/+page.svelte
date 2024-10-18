<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { type Package } from "$lib/types";

  let packages: { [name: string]: Package } = $state({});
  let isDockerRunning: boolean | null = $state(null);
  let installedPackages: Set<string> = $state(new Set());
  let installLoading: string | null = $state(null); // Track which package is being installed
  let deleteLoading: string | null = $state(null); // Track which package is being deleted
  let ready = $state(false); // Track when state checks are complete

  async function loadPackages() {
    try {
      packages = await invoke("get_packages");
      if (false) {
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

  async function deletePackage(name: string) {
    deleteLoading = name;
    try {
      await invoke("delete_package", { name });
      await loadPackages();
    } catch (error) {
      alert(`Failed to delete ${name}.`);
      console.error(error);
    } finally {
      deleteLoading = null;
    }
  }

  async function checkDocker() {
    try {
      isDockerRunning = await invoke("is_docker_running");
    } catch (error) {
      alert("Failed to check Docker status.");
      console.error(error);
    }
  }

  onMount(async () => {
    await checkDocker();
    await loadPackages();
  });
</script>

{#if Object.keys(packages).length > 0}
  {#each Object.entries(packages).sort(([a], [b]) => a.localeCompare(b)) as [name, p]}
    <article>
      <h3>{name}</h3>
      {#if !isDockerRunning}
        <p><u>Turn on Docker to install or delete packages.</u></p>
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
          onclick={() => deletePackage(name)}
          disabled={!isDockerRunning || deleteLoading === name}
        >
          {deleteLoading === name ? "Deleting..." : "Delete"}
        </button>
      {/if}
    </article>
  {/each}
{/if}
