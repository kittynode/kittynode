<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { type Package } from "$lib/types";

  let packages: { [name: string]: Package } = {};
  let isDockerRunning: null | boolean = null;

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

  async function checkDocker() {
    try {
      isDockerRunning = await invoke("is_docker_running");
    } catch (error) {
      console.error("Failed to check Docker", error);
    }
  }

  onMount(() => {
    checkDocker();
    getPackages();
  });
</script>

{#if Object.keys(packages).length > 0}
  {#each Object.entries(packages).sort(([nameA], [nameB]) => nameA.localeCompare(nameB)) as [name, p]}
    <article>
      <h3>{name}</h3>
      {#if !isDockerRunning}
        <p><mark>Turn on Docker to install this package.</mark></p>
      {/if}
      <p>{p.description}</p>
      <button on:click={() => installPackage(name)} disabled={!isDockerRunning}>Install</button>
    </article>
  {/each}
{/if}
