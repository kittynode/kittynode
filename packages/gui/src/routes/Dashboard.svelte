<script lang="ts">
  import type { Package } from "$lib/types";
  export let installedPackages: Package[];
</script>

{#if installedPackages.length === 0}
  <p>You have no packages installed.</p>
{:else}
  <ul>
    {#each installedPackages as pkg}
      <li>{pkg.name}</li>
    {/each}
  </ul>
{/if}
