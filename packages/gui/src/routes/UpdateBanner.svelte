<script>
import { Button } from "$lib/components/ui/button";
import { Separator } from "$lib/components/ui/separator";
import { updates } from "$stores/updates.svelte";
import { onMount } from "svelte";

function handleUpdate() {
  updates.installUpdate();
}

function handleDismiss() {
  updates.dismiss();
}

onMount(async () => {
  await updates.getUpdate();
});
</script>

{#if updates.hasUpdate && !updates.isDismissed}
  <div style="display: flex; justify-content: space-between; align-items: center;">
    <strong>A new update is available! ✨</strong>
    <div>
      <Button
        onclick={handleUpdate}
        disabled={updates.isProcessing}
      >
        {updates.isProcessing ? 'Updating...' : 'Update Now'}
      </Button>
      <Button
        onclick={handleDismiss}
        variant="secondary"
        disabled={updates.isProcessing}
      >
        Dismiss
      </Button>
    </div>
  </div>
  <Separator style="margin-top: 20px; margin-bottom: 20px;" />
{/if}
