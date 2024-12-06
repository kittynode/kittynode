<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { initializedStore } from "$stores/initialized.svelte";
import { Button } from "$lib/components/ui/button";
import { platform } from "@tauri-apps/plugin-os";
import { onMount } from "svelte";
import { remoteAccessStore } from "$stores/remoteAccess.svelte";
import { serverUrlStore } from "$stores/serverUrl.svelte";
import { updates } from "$stores/updates.svelte";
import { LoaderCircle } from "lucide-svelte";
import { refetchStores } from "$utils/refetchStores";
import { error } from "$utils/error";

let currentPlatform = $state("");

async function enableRemoteAccess() {
  try {
    remoteAccessStore.enable();
    alert("Remote access has been enabled.");
  } catch (e) {
    error(`Failed to enable remote access: ${e}`);
  }
}

async function disableRemoteAccess() {
  try {
    remoteAccessStore.disable();
    alert("Remote access has been disabled.");
  } catch (e) {
    error(`Failed to disable remote access: ${e}`);
  }
}

async function connectRemote() {
  try {
    setRemote("http://merlin:3000");
    alert("Connected to remote.");
  } catch (e) {
    error(`Failed to connect to remote: ${e}`);
  }
}

async function disconnectRemote() {
  try {
    setRemote("");
    alert("Disconnected from remote.");
  } catch (e) {
    error(`Failed to disconnect from remote: ${e}`);
  }
}

async function deleteKittynode() {
  try {
    await invoke("delete_kittynode", { serverUrl: serverUrlStore.serverUrl });
    await initializedStore.uninitialize();
    console.info("Kittynode data has been deleted successfully.");
  } catch (e) {
    error(`Failed to delete Kittynode: ${e}`);
  }
}

async function handleUpdate() {
  await updates.installUpdate();
}

function setRemote(serverUrl: string) {
  serverUrlStore.setServerUrl(serverUrl);
  // Refetch store caches
  refetchStores();
}

onMount(async () => {
  currentPlatform = platform();
});
</script>

<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight mb-4">Settings</h3>

<ul class="settings-list">
  {#if remoteAccessStore.remoteAccess === null}
    <li>Loading remote access status...</li>
  {:else if !remoteAccessStore.remoteAccess}
    <li>
      <span>Enable remote access</span>
      <Button onclick={enableRemoteAccess} disabled={ ["ios", "android"].includes(currentPlatform) }>Enable</Button>
    </li>
    <hr />
  {:else}
    <li>
      <span>Disable remote access</span>
      <Button onclick={disableRemoteAccess}>Disable</Button>
    </li>
    <hr />
  {/if}
  {#if serverUrlStore.serverUrl === ""}
    <li>
      <span>Connect remote kitty</span>
      <Button onclick={connectRemote}>Connect</Button>
    </li>
    <hr />
  {:else}
    <li>
      <span>Disconnect remote kitty</span>
      <Button onclick={disconnectRemote}>Disconnect</Button>
    </li>
    <hr />
  {/if}
  {#if !["ios", "android"].includes(currentPlatform)}
    <li>
      <span>Update Kittynode</span>
      <Button disabled={updates.isProcessing || !updates.hasUpdate} onclick={handleUpdate}>
        {#if updates.isProcessing}
          <LoaderCircle class="animate-spin" />
          Updating
        {:else if !updates.hasUpdate}
          No update available
        {:else}
          Update
        {/if}
      </Button>
    </li>
    <hr />
  {/if}
  <li>
    <span>Delete all Kittynode data</span>
    <Button onclick={deleteKittynode} disabled={serverUrlStore.serverUrl !== ""} variant="destructive">Delete data</Button>
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
