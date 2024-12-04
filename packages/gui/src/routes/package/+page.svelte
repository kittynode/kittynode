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

// Core package state
let installedPackages: Package[] = $state([]);

// Installation action states
let installLoading: string | null = $state(null);
let deleteLoading: string | null = $state(null);

// Logs state - single source of truth for which logs are being viewed
let activeLogType = $state<null | "execution" | "consensus">(null);

// Derived states
let canInstall = $derived(
  selectedPackageStore.package &&
    dockerStatus.isRunning &&
    !installedPackages.some(
      (p) => p.name === selectedPackageStore.package?.name,
    ) &&
    !installLoading &&
    !deleteLoading,
);

let isInstalled = $derived(
  selectedPackageStore.package &&
    installedPackages.some(
      (p) => p.name === selectedPackageStore.package?.name,
    ),
);

let canShowLogs = $derived(
  selectedPackageStore.package &&
    installedPackages.some(
      (p) => p.name === selectedPackageStore.package?.name,
    ),
);

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
  if (!dockerStatus.isRunning) {
    alert("Docker must be running to install packages.");
    return;
  }

  installLoading = name;
  try {
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

async function deletePackage(name: string) {
  if (!dockerStatus.isRunning) {
    alert("Docker must be running to delete packages.");
    return;
  }

  deleteLoading = name;
  try {
    await invoke("delete_package", {
      name,
      includeImages: false,
      serverUrl: serverUrlStore.serverUrl,
    });
    await loadInstalledPackages();
    alert(`Successfully deleted ${name}.`);
    // Clear logs view if package is deleted
    activeLogType = null;
  } catch (e) {
    error(`Failed to delete ${name}.`);
  } finally {
    deleteLoading = null;
  }
}

function toggleLogs(logType: "execution" | "consensus") {
  activeLogType = activeLogType === logType ? null : logType;
}

// Navigation effect
$effect(() => {
  if (!selectedPackageStore.package) {
    goto("/");
  }
});

onMount(async () => {
  dockerStatus.startPolling();
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

    {#if !isInstalled}
        <Button
            onclick={() => installPackage(pkg.name)}
            disabled={!canInstall}
        >
            {installLoading === pkg.name ? "Installing..." : "Install"}
        </Button>
    {:else}
        <Button
            variant="destructive"
            onclick={() => deletePackage(pkg.name)}
            disabled={!dockerStatus.isRunning || deleteLoading === pkg.name}
        >
            {deleteLoading === pkg.name ? "Deleting..." : "Delete"}
        </Button>
    {/if}

    <br />

    <!-- Logging -->
    {#if isInstalled}
        <h3 class="scroll-m-20 text-2xl font-semibold tracking-tight my-4">
            Logging
        </h3>
        <div class="flex gap-2">
            <Button
                variant="default"
                onclick={() => toggleLogs('execution')}
                disabled={!canShowLogs}
            >
                {activeLogType === 'execution' ? 'Hide execution logs' : 'View execution logs'}
            </Button>

            <Button
                variant="default"
                onclick={() => toggleLogs('consensus')}
                disabled={!canShowLogs}
            >
                {activeLogType === 'consensus' ? 'Hide consensus logs' : 'View consensus logs'}
            </Button>
        </div>

        {#if activeLogType === 'execution'}
            <p class="mt-4">Execution logs:</p>
            <div class="mt-4">
                <DockerLogs containerName="reth-node" tailLines={1000} />
            </div>
        {/if}

        {#if activeLogType === 'consensus'}
            <p class="mt-4">Consensus logs:</p>
            <div class="mt-4">
                <DockerLogs containerName="lighthouse-node" tailLines={1000} />
            </div>
        {/if}
    {/if}
{/if}
