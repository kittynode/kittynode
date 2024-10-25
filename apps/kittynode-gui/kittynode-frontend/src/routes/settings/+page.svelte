<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";
import { initializedStore } from "../../stores/initialized.svelte";

async function connectMobile() {
  await message("Coming soon.");
}

async function deleteKittynode() {
  try {
    await invoke("delete_kittynode");
    await initializedStore.uninitialize();
    message("Kittynode data has been deleted successfully.");
  } catch (error) {
    alert("Failed to delete Kittynode.");
    console.error(error);
  }
}
</script>

<h2>Settings</h2>

<ul class="settings-list">
  <li>
    <span>Connect your mobile device</span>
    <button onclick={connectMobile}>Connect mobile</button>
  </li>
  <hr />
  <li>
    <span>Delete all Kittynode data</span>
    <button onclick={deleteKittynode}>Delete data</button>
  </li>
</ul>

<style>
  .settings-list {
    list-style: none;
    padding: 0;
  }

  .settings-list li {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
</style>
