<script lang="ts">
import { initializedStore } from "../stores/initialized.svelte";
import { goto } from "$app/navigation";
import { platform } from '@tauri-apps/plugin-os';
import { onMount } from "svelte";
import { invoke } from "@tauri-apps/api/core";

let currentPlatform = $state("");

async function initKittynode() {
  await initializedStore.initialize();
  await goto("/");
}

async function installPackage(name: string) {
  try {
    await invoke("install_package", { name });
    alert("did the install woot");
  } catch (error) {
    alert(`Failed to install ${name} with this error: ${error}`);
    console.error(error);
  }
}

onMount(async () => {
  currentPlatform = platform();
});
</script>

<main>
  <img class="logo" src="/images/kittynode-light.png" alt="Kittynode Logo">
  {#if currentPlatform === "ios"}
  <button onclick={() => installPackage("Ethereum")}>Super secret iOS button installs a package</button>
  {:else}
  <button onclick={initKittynode}>Get Started</button>
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
