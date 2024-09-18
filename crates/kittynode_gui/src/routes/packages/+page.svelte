<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { type Package } from "$lib/types";

  let packages: Record<string, Package> = {};

  async function getPackages() {
    try {
      packages = await invoke("get_packages");
    } catch (error) {
      console.error("Failed to fetch packages", error);
    }
  }

  onMount(() => {
    getPackages();
  });
</script>

{#if Object.keys([packages]).length > 0}
  {#each Object.keys(packages) as key}
    <article>
      <h3>{key}</h3>
      <p>Version: {packages[key].package.version}</p>
      <button>Install {key}</button>
    </article>
  {/each}
{/if}
