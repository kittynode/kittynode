<script lang="ts">
import { initializedStore } from "../stores/initialized.svelte";
import { goto } from "$app/navigation";
import { platform } from "@tauri-apps/plugin-os";
import { onMount } from "svelte";
import { Button } from "$lib/components/ui/button";

let currentPlatform = $state("");

async function initKittynode() {
  if (currentPlatform === "ios") {
    await initializedStore.cheatInitialize();
  } else {
    await initializedStore.initialize();
  }
  await goto("/");
}

onMount(async () => {
  currentPlatform = platform();
});
</script>

<main class="flex flex-col justify-center items-center h-full text-center p-4">
  <Button onclick={initKittynode}>Get started</Button>
</main>
