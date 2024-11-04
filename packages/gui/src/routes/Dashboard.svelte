<script lang="ts">
export let installedPackages: Set<string>;
</script>

{#if installedPackages.size === 0}
  <p>You have no packages installed.</p>
{:else}
<ul>
{#each installedPackages as name}
  <li>{name}</li>
{/each}
</ul>
{/if}
