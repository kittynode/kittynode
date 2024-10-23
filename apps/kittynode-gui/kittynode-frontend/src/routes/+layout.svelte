<script lang="ts">
import "@picocss/pico/css/pico.purple.min.css";
import { page } from "$app/stores";
import { onMount } from "svelte";
import { win } from "../stores/window.svelte.ts";
import { invoke } from "@tauri-apps/api/core";
import { Welcome } from "../components";

const { children } = $props();
let isInitialized = $state(false);

async function getIsInitialized() {
  try {
    isInitialized = await invoke("is_initialized");
  } catch (error) {
    alert("Failed to check if Kittynode is initialized.");
    console.error(error);
  }
}

onMount(async () => {
  await getIsInitialized();
  win.show();
});
</script>

{#if !isInitialized}
<Welcome />
{:else}
<header class="container">
  <nav>
    <ul>
      <li>
        <strong>
          Kittynode
        </strong>
      </li>
    </ul>
    <ul>
      <li>
        <a href="/" aria-current={$page.url.pathname === "/"}>Home</a>
      </li>
      <li>
        <a href="/packages" aria-current={$page.url.pathname === "/packages"}>Packages</a>
      </li>
      <li>
        <a href="/system-info" aria-current={$page.url.pathname === "/system-info"}>System Info</a>
      </li>
      <li>
        <a href="/settings" aria-current={$page.url.pathname === "/settings"}>Settings</a>
      </li>
    </ul>
  </nav>
</header>

<main class="container" style="margin-top: 1rem;">
  {@render children()}
</main>
{/if}
