<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { type Package } from "$lib/types";

  let packages: { [name: string]: Package } = {};

  async function getPackages() {
    try {
      packages = await invoke("get_packages");
    } catch (error) {
      console.error("Failed to fetch packages", error);
    }
  }

  async function installPackage(name: string) {
    try {
        await invoke("install_package", { name });
    } catch (error) {
      console.error("Failed to install package", error);
    }
  }

  onMount(() => {
    getPackages();
  });
</script>

{#if Object.keys(packages).length > 0}
  {#each Object.entries(packages).sort(([nameA], [nameB]) => nameA.localeCompare(nameB)) as [name, p]}
    <article>
      <h3>{name}</h3>
      <p>{p.description}</p>
      <button on:click={() => installPackage(name)}>Install</button>
    </article>
  {/each}
{/if}
