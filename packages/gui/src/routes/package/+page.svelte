<script lang="ts">
import Button from "$lib/components/ui/button/button.svelte";
import Link from "$lib/components/ui/link/link.svelte";
import type { Package } from "$lib/types";
import { serverUrlStore } from "$stores/serverUrl.svelte";
import { selectedPackageStore } from "$stores/selectedPackage.svelte";
import { invoke } from "@tauri-apps/api/core";
import { onDestroy, onMount } from "svelte";
import DockerLogs from "./DockerLogs.svelte";
import { goto } from "$app/navigation";
import { dockerStatus } from "$stores/dockerStatus.svelte";
import { error } from "$utils/error";

let showLogsExecution = $state(false);
let showLogsConsensus = $state(false);

let installedPackages: Package[] = $state([]);
let installLoading: string | null = $state(null);
let deleteLoading: string | null = $state(null);

async function loadInstalledPackages() {
  try {
    if (dockerStatus.isRunning) {
      installedPackages = await invoke("get_installed_packages", {
        serverUrl: serverUrlStore.serverUrl,
      });
    }
  } catch (e) {
    error(`Failed to load packages: ${e}`);
  }
}

async function installPackage(name: string) {
  installLoading = name;
  try {
    // Check Docker status right before installation
    const isRunning = await invoke("is_docker_running");
    if (!isRunning) {
      alert("Docker must be running to install packages.");
      return;
    }

    await invoke("install_package", {
      name,
      serverUrl: serverUrlStore.serverUrl,
    });
    await loadInstalledPackages();
    alert(`Successfully installed ${name}.`);
  } catch (e) {
    error(`Failed to install ${name}.`);
  } finally {
    installLoading = null;
  }
}

async function deletePackage(name: string, includeImages: boolean) {
  deleteLoading = name;
  try {
    // Check Docker status right before deletion
    const isRunning = await invoke("is_docker_running");
    if (!isRunning) {
      alert("Docker must be running to delete packages.");
      return;
    }

    await invoke("delete_package", {
      name,
      includeImages,
      serverUrl: serverUrlStore.serverUrl,
    });
    await loadInstalledPackages();
    alert(`Successfully deleted ${name}.`);
  } catch (e) {
    error(`Failed to delete ${name}.`);
  } finally {
    deleteLoading = null;
  }
}

// Redirect if no package is selected
$effect(() => {
  if (!selectedPackageStore.package) {
    goto("/");
  }
});

onMount(async () => {
  // start docker status polling
  dockerStatus.startPolling();

  // load installed packages
  await loadInstalledPackages();
});

onDestroy(() => {
  selectedPackageStore.clear();
  dockerStatus.stopPolling();
});
</script>

<Link href="/" text="← Back home" />

{#if selectedPackageStore.package}
  {@const pkg = selectedPackageStore.package}

  <!-- Overview -->
  <h3 class="scroll-m-20 text-2xl font-semibold tracking-tight my-4">
    Overview
  </h3>
  <p><strong>Name:</strong> {pkg.name}</p>
  <p><strong>Network:</strong> Holesky</p>

  <!-- Lifecycle -->
  <h3 class="scroll-m-20 text-2xl font-semibold tracking-tight my-4">
    Lifecycle
  </h3>
  {#if !dockerStatus.isRunning}
    <p class="font-bold">
        Turn on Docker to use this package. If you need to install
        Docker, please follow the installation guide <Link href="https://docs.docker.com/engine/install/" targetBlank text="here" />.
    </p>
    <br />
  {/if}
  <Button
    onclick={() => installPackage(pkg.name)}
    disabled={
      !dockerStatus.isRunning ||
      installedPackages.some((installed) => installed.name === pkg.name) ||
      installLoading === pkg.name ||
      deleteLoading === pkg.name}
  >
    {installLoading === pkg.name ? "Installing..." : "Install"}
  </Button>

  {#if installedPackages.some((installed) => installed.name === pkg.name)}
    <Button
      variant="destructive"
      onclick={() => deletePackage(pkg.name, false)}
      disabled={!dockerStatus.isRunning || deleteLoading === pkg.name}
    >
      {deleteLoading === pkg.name ? "Deleting..." : "Delete"}
    </Button>
  {/if}

  <br />

  <!-- Logging -->
  <h3 class="scroll-m-20 text-2xl font-semibold tracking-tight my-4">
    Logging
  </h3>
  <Button
      disabled={!installedPackages.some((installed) => installed.name === pkg.name)}
      onclick={() => showLogsExecution = !showLogsExecution}
  >
      {showLogsExecution ? 'Hide execution logs' : 'View execution logs'}
  </Button>

  <Button
      disabled={!installedPackages.some((installed) => installed.name === pkg.name)}
      onclick={() => showLogsConsensus = !showLogsConsensus}
  >
      {showLogsConsensus ? 'Hide consensus logs' : 'View consensus logs'}
  </Button>

  {#if showLogsExecution}
      <p class="mt-4">Execution logs:</p>
      <div class="mt-4">
          <DockerLogs containerName="reth-node" tailLines={1000} />
      </div>
  {/if}

  {#if showLogsConsensus}
    <p class="mt-4">Consensus logs:</p>
      <div class="mt-4">
          <DockerLogs containerName="lighthouse-node" tailLines={1000} />
      </div>
  {/if}
{/if}
