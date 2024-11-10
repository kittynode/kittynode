<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";
import { initializedStore } from "../../stores/initialized.svelte";
import { Button } from "$lib/components/ui/button";
import { platform } from "@tauri-apps/plugin-os";
import { onMount } from "svelte";

let currentPlatform = $state("");
let isRemoteMode: boolean | null = $state(null);

async function doThing() {
  try {
    const resp = await invoke("set_is_remote_flag", { value: false });
    const anotherResp = await invoke("get_server_url");
    alert(`Success: ${anotherResp}`);
  } catch (error) {
    alert(`Error: ${error}`);
    console.error("Failed to do thing:", error);
  }
}

async function connect() {
  await message("Coming soon.");
}

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
    isRemoteMode = ((await invoke("get_capabilities")) as string[]).includes(
      "remote_control",
    );
  } catch (error) {
    alert(`Error: ${error}`);
    console.error("Failed to update remote control capability:", error);
  }
}

async function deleteKittynode() {
  try {
    await invoke("delete_kittynode");
    await initializedStore.uninitialize();
    message("Kittynode data has been deleted successfully.");
  } catch (error) {
    alert(`Failed to delete Kittynode: ${error}`);
    console.error(error);
  }
}

onMount(async () => {
  currentPlatform = platform();
  isRemoteMode = ((await invoke("get_capabilities")) as string[]).includes(
    "remote_control",
  );
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
  {#if isRemoteMode === null}
    <li>Loading remote control status...</li>
  {:else if isRemoteMode}
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
  <hr />
  <li>
    <span>Do thing</span>
    <Button onclick={doThing}>Do thing</Button>
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
