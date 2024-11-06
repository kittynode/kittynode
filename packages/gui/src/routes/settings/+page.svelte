<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { message } from "@tauri-apps/plugin-dialog";
  import { initializedStore } from "../../stores/initialized.svelte";
  import { Button } from "$lib/components/ui/button";
  import { platform } from "@tauri-apps/plugin-os";
  import { onMount } from "svelte";

  let currentPlatform = $state("");

  async function connectMobile() {
    await message("Coming soon.");
  }

  async function deleteKittynode() {
    try {
      if (currentPlatform !== "ios") {
        await invoke("delete_kittynode");
      }
      await initializedStore.uninitialize();
      message("Kittynode data has been deleted successfully.");
    } catch (error) {
      alert("Failed to delete Kittynode.");
      console.error(error);
    }
  }

  onMount(async () => {
    currentPlatform = platform();
  });
</script>

<ul class="settings-list">
  <li>
    <span>Connect your mobile device</span>
    <Button onclick={connectMobile}>Connect mobile</Button>
  </li>
  <hr />
  <li>
    <span>Delete all Kittynode data</span>
    <Button onclick={deleteKittynode}>Delete data</Button>
  </li>
</ul>

<style>
  hr {
    margin: 8px 0px 8px 0px;
  }

  .settings-list li {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
</style>
