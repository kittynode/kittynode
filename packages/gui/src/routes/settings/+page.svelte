<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";
import { initializedStore } from "$stores/initialized.svelte";
import { Button } from "$lib/components/ui/button";
import { platform } from "@tauri-apps/plugin-os";
import { onMount } from "svelte";
import { remoteAccessStore } from "$stores/remoteAccess.svelte";
import { serverUrlStore } from "$stores/serverUrl.svelte";
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

let currentPlatform = $state("");

async function enableRemoteAccess() {
  try {
    remoteAccessStore.enable();
    await message("Remote access has been enabled.");
  } catch (e) {
    alert(`Failed to enable remote access: ${e}`);
  }
}

async function disableRemoteAccess() {
  try {
    remoteAccessStore.disable();
    await message("Remote access has been disabled.");
  } catch (e) {
    alert(`Failed to disable remote access: ${e}`);
  }
}

async function connectRemote() {
  try {
    const serverUrl = "http://merlin:3000";
    serverUrlStore.setServerUrl(serverUrl);
    await message("Connected to remote.");
  } catch (e) {
    alert(`Failed to connect to remote: ${e}`);
  }
}

async function disconnectRemote() {
  try {
    serverUrlStore.setServerUrl("");
    await message("Disconnected from remote.");
  } catch (e) {
    alert(`Failed to disconnect from remote: ${e}`);
  }
}

async function deleteKittynode() {
  try {
    await invoke("delete_kittynode", { serverUrl: serverUrlStore.serverUrl });
    await initializedStore.uninitialize();
    message("Kittynode data has been deleted successfully.");
  } catch (error) {
    alert(`Failed to delete Kittynode: ${error}`);
    console.error(error);
  }
}

async function updateKittynode() {
  try {
    const update = await check();
    if (update) {
      console.log(
        `found update ${update.version} from ${update.date} with notes ${update.body}`,
      );
      let downloaded = 0;
      let contentLength = 0;

      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            contentLength = event.data.contentLength as number;
            console.log(
              `started downloading ${event.data.contentLength} bytes`,
            );
            break;
          case "Progress":
            downloaded += event.data.chunkLength;
            console.log(`downloaded ${downloaded} from ${contentLength}`);
            break;
          case "Finished":
            console.log("download finished");
            break;
        }
      });

      alert("Update successfully installed!");
      console.log("update installed");
      await relaunch();
    } else {
      alert("No update available.");
    }
  } catch (error) {
    alert(`Failed to update Kittynode: ${error}`);
    console.error(error);
  }
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
      <Button onclick={updateKittynode}>Update</Button>
    </li>
    <hr />
  {/if}
  <li>
    <span>Delete all Kittynode data</span>
    <Button onclick={deleteKittynode} disabled={serverUrlStore.serverUrl !== ""}>Delete data</Button>
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
