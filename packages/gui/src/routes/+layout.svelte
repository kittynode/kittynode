<script lang="ts">
import "../app.css";
import { onMount } from "svelte";
import { windowShownStore } from "../stores/windowShown.svelte.ts";
import { initializedStore } from "../stores/initialized.svelte";
import { platform } from "@tauri-apps/plugin-os";
import { ModeWatcher } from "mode-watcher";
import Welcome from "./Welcome.svelte";
import Navigation from "./Navigation.svelte";

const { children } = $props();

onMount(async () => {
  await windowShownStore.show();
});
</script>

<ModeWatcher />
{#if !initializedStore.initialized}
  <Welcome />
{:else}
  <div class="flex flex-col h-screen">
    <main class="flex-1 overflow-y-auto pb-24">
      <div class="container mx-auto pt-8">
        {@render children()}
      </div>
    </main>
    <div class="fixed bottom-0 left-0 right-0 bg-background border-t">
      <div class="container mx-auto py-4">
        <Navigation />
      </div>
    </div>
  </div>
{/if}

<style>
  :global(html, body) {
    height: 100%;
    margin: 0;
    padding: 0;
    overflow: hidden;
  }
</style>
