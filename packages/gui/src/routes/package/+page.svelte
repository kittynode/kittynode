<script lang="ts">
import Button from "$lib/components/ui/button/button.svelte";
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
<!-- <div class="mt-4">
  <p>Package is not installed.</p>
</div> -->

<!-- Overview -->
<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight my-4">
  Overview
</h3>
<p><strong>Name:</strong> Ethereum</p>
<p><strong>Network:</strong> Holesky</p>
<p><strong>Latest block hash:</strong> <a href="/" class="text-primary font-medium underline underline-offset-4">0x1234...6789</a></p>
<Button class="mt-4">View logs</Button>

<!-- Configuration -->
<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight my-4">
  Configuration
</h3>
<p>Here we will show the entire config and map it into a Svelte form.</p>
<Button class="mt-4">Update configuration</Button>

<!-- Validator setup -->
<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight my-4">
  Validator
</h3>
<p>Here we will walk through an entire validator setup workflow:</p>
<ul>
  <li>&nbsp;&nbsp;&nbsp;&nbsp;- Generating the cryptographic key pair (using the key manager).</li>
  <li>&nbsp;&nbsp;&nbsp;&nbsp;- Generating and uploading deposit data (including wallet connection and local tx submission).</li>
  <li>&nbsp;&nbsp;&nbsp;&nbsp;- Displaying monitoring logs and configurability after setup.</li>
</ul>
<Button class="mt-4">Set up validator</Button>

<!-- Lifecycle -->
<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight my-4">
  Lifecycle
</h3>
<Button class="mt-4">Restart node</Button>
<Button class="mt-4" variant="destructive">Delete</Button>

{/if}
