<script lang="ts">
import "../app.css";
import { onMount } from "svelte";
import { windowShownStore } from "../stores/windowShown.svelte.ts";
import { initializedStore } from "../stores/initialized.svelte";
import { platform } from "@tauri-apps/plugin-os";
import Welcome from "./Welcome.svelte";
import Navigation from "./Navigation.svelte";

const { children } = $props();

onMount(async () => {
  await windowShownStore.show();
});
</script>

{#if !initializedStore.initialized}
  <Welcome />
{:else}
  <div class="container mx-auto pt-8">
    {@render children()}
  </div>
  <div class="container mx-auto fixed {["ios", "android"].includes(platform()) ? 'bottom-4' : 'bottom-8'} left-0 right-0">
    <Navigation />
  </div>
{/if}

<style>
  :global(html, body) {
    height: 100%;
    margin: 0;
    padding: 0;
  }
</style>
