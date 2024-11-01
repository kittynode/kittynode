<script lang="ts">
import { initializedStore } from "../stores/initialized.svelte";
import { goto } from "$app/navigation";
import { platform } from "@tauri-apps/plugin-os";
import { onMount } from "svelte";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "$lib/components/ui/button/index.js";

let currentPlatform = $state("");

async function initKittynode() {
  await initializedStore.initialize();
  await goto("/");
}

async function installPackage(name: string) {
  try {
    await invoke("install_package", { name });
    alert(`Successfully installed: ${name}`);
  } catch (error) {
    alert(`Failed to install ${name}: ${error}`);
  }
}

async function deletePackage(name: string) {
  try {
    await invoke("delete_package", { name, includeImages: false });
    alert(`Successfully deleted: ${name}`);
  } catch (error) {
    alert(`Failed to delete ${name}: ${error}`);
  }
}

onMount(async () => {
  currentPlatform = platform();
});
</script>

<main>
  <img class="logo" src="/images/kittynode-light.png" alt="Kittynode Logo">
  {#if currentPlatform === "ios"}
  <Button onclick={() => installPackage("Ethereum")}>Install Ethereum</Button>
  <br />
  <Button onclick={() => deletePackage("Ethereum")}>Delete Ethereum</Button>
  {:else}
  <Button onclick={initKittynode}>Get Started</Button>
  {/if}
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 100vh;
    text-align: center;
  }

  /* Default logo for light mode */
  img.logo {
    width: 300px; /* Adjust logo size */
    margin-bottom: 20px;
    vertical-align: middle;
    content: url("/images/kittynode-light.png");
  }

  /* Change logo in dark mode */
  @media (prefers-color-scheme: dark) {
    img.logo {
    content: url("/images/kittynode-dark.png");
    }
  }
</style>
