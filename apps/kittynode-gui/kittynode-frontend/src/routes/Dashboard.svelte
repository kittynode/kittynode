<script lang="ts">
export let installedPackages: Set<string>;
</script>

{#if installedPackages.size === 0}
  <p>You have no packages installed.</p>
{:else}
{#each installedPackages as name}
  <p>{name}</p>
{/each}
{/if}
