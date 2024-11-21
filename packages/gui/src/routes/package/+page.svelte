<script lang="ts">
import type { Package } from "$lib/types";
import { serverUrlStore } from "$stores/serverUrl.svelte";
import { invoke } from "@tauri-apps/api/core";
import { onMount } from "svelte";

let installedPackages = $state<Package[]>([]);
let pkg = $state<Package | null>(null);

onMount(async () => {
  installedPackages = await invoke("get_installed_packages", {
    serverUrl: serverUrlStore.serverUrl,
  });
  if (installedPackages.length === 1) {
    pkg = installedPackages[0];
  }
});
</script>

<a href="/" class="text-primary font-medium underline underline-offset-4">← Back home</a>


{#if pkg}
<div class="mt-4">
  <p>Name: {pkg.name}</p>
</div>
{:else}
<div class="mt-4">
  <p>No packages installed.</p>
</div>
{/if}
