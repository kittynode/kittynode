<script lang="ts">
import { onMount, onDestroy } from "svelte";
import { invoke } from "@tauri-apps/api/core";
import type { Package } from "$lib/types";
import { Button } from "$lib/components/ui/button";
import * as Card from "$lib/components/ui/card/index.js";
import { platform } from "@tauri-apps/plugin-os";
import { serverUrlStore } from "$stores/serverUrl.svelte";
import { systemInfoStore } from "$stores/systemInfo.svelte";
import { dockerStatus } from "$stores/dockerStatus.svelte";
import { goto } from "$app/navigation";
import { error } from "$utils/error";
import Link from "$lib/components/ui/link/link.svelte";

let packages: { [name: string]: Package } = $state({});
let installedPackages: Package[] = $state([]);
let installLoading: string | null = $state(null);
let deleteLoading: string | null = $state(null);

async function loadPackages() {
  try {
    packages = await invoke("get_packages");
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
    await loadPackages();
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
    await loadPackages();
    alert(`Successfully deleted ${name}.`);
  } catch (e) {
    error(`Failed to delete ${name}.`);
  } finally {
    deleteLoading = null;
  }
}

function isMobileAndLocal() {
  return (
    ["ios", "android"].includes(platform()) && serverUrlStore.serverUrl === ""
  );
}

onMount(async () => {
  // prefetch stores async
  if (!systemInfoStore.systemInfo) systemInfoStore.fetchSystemInfo();

  // start docker status polling
  dockerStatus.startPolling();

  // do not fetch if mobile and local
  if (!isMobileAndLocal()) {
    await loadPackages();
  }
});

onDestroy(() => {
  dockerStatus.stopPolling();
});

// reload packages when Docker becomes available
$effect(() => {
  if (dockerStatus.isRunning && !isMobileAndLocal()) {
    loadPackages();
  }
});
</script>

<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight mb-4">
  Home
</h3>

{#if Object.keys(packages).length > 0}
  {#each Object.entries(packages).sort( ([a], [b]) => a.localeCompare(b), ) as [name, p]}
    <Card.Root>
      <Card.Header>
        <Card.Title>{name}</Card.Title>
        <Card.Description>
          {#if !dockerStatus.isRunning}
            <p class="font-bold">
                Turn on Docker to use this package. If you need to install
                Docker, please follow the installation guide <Link href="https://docs.docker.com/engine/install/" targetBlank text="here" />.
            </p>
          {/if}
          {p.description}
        </Card.Description>
      </Card.Header>
      <Card.Content>
        <Button
          onclick={() => installPackage(name)}
          disabled={
            !dockerStatus.isRunning ||
            installedPackages.some((pkg) => pkg.name === name) ||
            installLoading === name ||
            deleteLoading === name}
        >
          {installLoading === name ? "Installing..." : "Install"}
        </Button>

        <Button
          variant="secondary"
          onclick={() => goto("package")}
          >Configure</Button>

        {#if installedPackages.some((pkg) => pkg.name === name)}
          <Button
            variant="destructive"
            onclick={() => deletePackage(name, false)}
            disabled={!dockerStatus.isRunning || deleteLoading === name}
          >
            {deleteLoading === name ? "Deleting..." : "Delete"}
          </Button>
        {/if}
      </Card.Content>
    </Card.Root>
  {/each}
{:else}
  <p>No packages available.</p>
{/if}
