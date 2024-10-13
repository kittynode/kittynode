<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { type Package } from "$lib/types";

  let packages: { [name: string]: Package } = {};
  let isDockerRunning: boolean | null = null;
  let installedPackages: Set<string> = new Set();
  let installLoading: string | null = null; // Track which package is being installed
  let deleteLoading: string | null = null; // Track which package is being deleted
  let ready = false; // Track when state checks are complete

  async function loadPackages() {
    try {
      packages = await invoke("get_packages");
      const installed = await invoke<string[]>("get_installed_packages");
      installedPackages = new Set(installed);
    } catch (error) {
      alert("Failed to load packages.");
      console.error(error);
    } finally {
      ready = true; // Mark the state as ready
    }
  }

  async function installPackage(name: string) {
    installLoading = name;
    try {
      await invoke("install_package", { name });
      await loadPackages(); // Refetch state after installation
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
      await loadPackages(); // Refetch state after deletion
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
        on:click={() => installPackage(name)}
        disabled={!ready || !isDockerRunning || installedPackages.has(name) || installLoading === name || deleteLoading === name}
      >
        {installLoading === name ? "Installing..." : "Install"}
      </button>

      {#if installedPackages.has(name)}
        <button
          class="secondary"
          on:click={() => deletePackage(name)}
          disabled={!isDockerRunning || deleteLoading === name}
        >
          {deleteLoading === name ? "Deleting..." : "Delete"}
        </button>
      {/if}
    </article>
  {/each}
{/if}
