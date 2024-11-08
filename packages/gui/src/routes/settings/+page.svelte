<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";
import { initializedStore } from "../../stores/initialized.svelte";
import { Button } from "$lib/components/ui/button";
import { platform } from "@tauri-apps/plugin-os";
import { onMount } from "svelte";

let currentPlatform = $state("");
let isRemoteMode = $state(false);

async function connectMobile() {
  await message("Coming soon.");
}

async function remoteControl() {
  try {
    if (isRemoteMode) {
      await invoke("remove_capability", { name: "remote_control" });
    } else {
      await invoke("add_capability", { name: "remote_control" });
    }
    isRemoteMode = (await invoke("get_capabilities")).includes("remote_control");
  } catch (error) {
    console.error("Failed to update remote control capability:", error);
  }
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
  currentPlatform = await platform();
  isRemoteMode = (await invoke("get_capabilities")).includes("remote_control");
});
</script>

<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight mb-4">Settings</h3>

<ul class="settings-list">
  {#if !["ios", "android"].includes(currentPlatform)}
    <li>
      <span>Connect your mobile device</span>
      <Button onclick={connectMobile}>Connect mobile</Button>
    </li>
    <hr />
  {/if}
  {#if isRemoteMode}
    <li>
      <span>Disable remote control</span>
      <Button onclick={remoteControl}>Disable</Button>
    </li>
    <hr />
  {:else}
    <li>
      <span>Enable remote control</span>
      <Button onclick={remoteControl}>Enable</Button>
    </li>
    <hr />
  {/if}
  <li>
    <span>Delete all Kittynode data</span>
    <Button onclick={deleteKittynode}>Delete data</Button>
  </li>
</ul>

<style>
  hr {
    margin: 16px 0px 16px 0px;
  }

  .settings-list li {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
</style>
