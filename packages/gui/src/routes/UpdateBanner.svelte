<script>
import { Button } from "$lib/components/ui/button";
import { Separator } from "$lib/components/ui/separator";
import { needsUpdateStore } from "$stores/needsUpdate.svelte";
import { onMount } from "svelte";

function handleUpdate() {
  alert("Update action triggered!");
}

function handleDismiss() {
  needsUpdateStore.dismiss();
}

onMount(async () => {
  await needsUpdateStore.getUpdate();
});
</script>

{#if needsUpdateStore.hasUpdate && !needsUpdateStore.isDismissed}
  <div style="display: flex; justify-content: space-between; align-items: center;">
    <strong>A new update is available! ✨</strong>
    <div>
      <Button onclick={handleUpdate}>Update Now</Button>
      <Button onclick={handleDismiss} variant="secondary">Dismiss</Button>
    </div>
  </div>
  <Separator style="margin-top: 20px; margin-bottom: 20px;" />
{/if}
