<script lang="ts">
import { initializedStore } from "$stores/initialized.svelte";
import { goto } from "$app/navigation";
import { platform } from "@tauri-apps/plugin-os";
import { onMount } from "svelte";
import { Button } from "$lib/components/ui/button";
import { mode } from "mode-watcher";

let currentPlatform = $state("");

async function initKittynode() {
  try {
    if (["ios", "android"].includes(currentPlatform)) {
      await initializedStore.fakeInitialize();
    } else {
      await initializedStore.initialize();
    }
  } catch (e) {
    alert(`Failed to initialize kittynode: ${e}`);
  }
  await goto("/");
}

onMount(async () => {
  currentPlatform = platform();
});
</script>

<main class="flex flex-col justify-center items-center h-full text-center p-4">
  {#if $mode}
    <img
      class="logo w-48 mb-4"
      src={`/images/kittynode-${$mode}.png`}
      alt="Kittynode Logo"
    />
  {/if}
  <Button onclick={initKittynode}>Get started</Button>
  <p>This is the super duper special kittynode with an update that works!</p>
</main>
