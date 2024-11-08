<script lang="ts">
import type { Package } from "$lib/types";
export let installedPackages: Package[];
</script>

{#if installedPackages.length === 0}
  <p>You have no packages installed.</p>
{:else}
  <ul>
    {#each installedPackages as pkg}
      <ul class="container mx-auto list-disc">
        <li>{pkg.name}</li>
      </ul>
    {/each}
  </ul>
{/if}
