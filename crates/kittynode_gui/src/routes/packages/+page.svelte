<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { type Package } from "$lib/types";

  let packages: Package[] = [];

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

{#if packages.length > 0}
  {#each packages as p}
    <article>
      <h3>{p.package.name}</h3>
      <p>Version: {p.package.version}</p>
      <button>Install {p.package.name}</button>
    </article>
  {/each}
{/if}
