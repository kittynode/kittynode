<script lang="ts">
import { onMount } from "svelte";
import { invoke } from "@tauri-apps/api/core";
import type { Package } from "$lib/types";
import Dashboard from "./Dashboard.svelte";
import { Button } from "$lib/components/ui/button";
import * as Card from "$lib/components/ui/card/index.js";
import { platform } from "@tauri-apps/plugin-os";
import { serverUrlStore } from "$stores/serverUrl.svelte";

let packages: { [name: string]: Package } = $state({});
let isDockerRunning: boolean | null = $state(null);
let installedPackages: Package[] = $state([]);
let installLoading: string | null = $state(null);
let deleteLoading: string | null = $state(null);
let currentPlatform = $state("");

async function loadPackages() {
  try {
    packages = await invoke("get_packages");
    if (isDockerRunning) {
      installedPackages = await invoke("get_installed_packages", {
        serverUrl: serverUrlStore.serverUrl,
      });
    }
  } catch (error) {
    alert(`Failed to load packages: ${error}`);
    console.error(error);
  }
}

async function installPackage(name: string) {
  installLoading = name;
  try {
    await invoke("install_package", {
      name,
      serverUrl: serverUrlStore.serverUrl,
    });
    await loadPackages();
    alert(`Successfully installed ${name}.`);
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
    await invoke("delete_package", {
      name,
      includeImages,
      serverUrl: serverUrlStore.serverUrl,
    });
    await loadPackages();
    alert(`Successfully deleted ${name}.`);
  } catch (error) {
    alert(`Failed to delete ${name}.`);
    console.error(error);
  } finally {
    deleteLoading = null;
  }
}

async function checkDocker() {
  isDockerRunning =
    currentPlatform === "ios" ? true : await invoke("is_docker_running");
}

onMount(async () => {
  currentPlatform = platform();
  await checkDocker();
  if (
    !(
      ["ios", "android"].includes(currentPlatform) &&
      serverUrlStore.serverUrl === ""
    )
  ) {
    await loadPackages();
  }
});
</script>

<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight mb-4">
  Dashboard
</h3>

<div class="mb-8">
  <Dashboard {installedPackages} />
</div>

<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight mb-4">
  Package store
</h3>

{#if serverUrlStore.serverUrl === "" && ["ios", "android"].includes(currentPlatform)}
  <Card.Root>
    <Card.Header>
      <Card.Title>Helios</Card.Title>
      <Card.Description>
        Runs a Helios light client on Holesky, using wasm.
      </Card.Description>
    </Card.Header>
    <Card.Content>
      <Button
        onclick={() => alert("Coming soon!")}
      >
       Install
      </Button>
    </Card.Content>
  </Card.Root>
{:else}
  {#if Object.keys(packages).length > 0}
    {#each Object.entries(packages).sort( ([a], [b]) => a.localeCompare(b), ) as [name, p]}
      <Card.Root>
        <Card.Header>
          <Card.Title>{name}</Card.Title>
          <Card.Description>
            {#if !isDockerRunning && currentPlatform !== "ios"}
              <p>
                <strong
                  >Turn on Docker to use this package. If you need to install
                  Docker, please follow the installation guide <a
                    href="https://docs.docker.com/engine/install/"
                    target="_blank">here</a
                  >.</strong
                >
              </p>
            {/if}
            {p.description}
          </Card.Description>
        </Card.Header>
        <Card.Content>
          <Button
            onclick={() => installPackage(name)}
            disabled={
              !isDockerRunning ||
              installedPackages.some((pkg) => pkg.name === name) ||
              installLoading === name ||
              deleteLoading === name}
          >
            {installLoading === name ? "Installing..." : "Install"}
          </Button>

          {#if installedPackages.some((pkg) => pkg.name === name)}
            <Button
              class="secondary"
              onclick={() => deletePackage(name, false)}
              disabled={!isDockerRunning || deleteLoading === name}
            >
              {deleteLoading === name ? "Deleting..." : "Delete"}
            </Button>
          {/if}
        </Card.Content>
      </Card.Root>
    {/each}
  {/if}
{/if}
