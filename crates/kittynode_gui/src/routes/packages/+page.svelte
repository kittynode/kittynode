<script lang="ts">
  import { onMount } from "svelte";
  import { invokeWithValidation } from "$lib/api";
  import { getPackagesSchema, type Package } from "$lib/schemas";

  let packages: Package[] = [];

  async function getPackages() {
    try {
      packages = await invokeWithValidation("get_packages", getPackagesSchema);
    } catch (error) {
      console.error("Failed to fetch packages", error);
    }
  }

  onMount(() => {
    getPackages();
  });
</script>

{#if packages.length > 0}
  {#each packages as pkg}
    <article>
      <h2>{pkg.package.name} - {pkg.package.version}</h2>
      <button>Install {pkg.package.name}</button>
    </article>
  {/each}
{/if}
