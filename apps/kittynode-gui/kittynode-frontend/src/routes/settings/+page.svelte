<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { message } from '@tauri-apps/plugin-dialog';

  async function connectMobile() {
    await message("Coming soon.");
  }

  async function deleteKittynode() {
    try {
      await invoke("delete_kittynode");
      window.location.reload();
    } catch (error) {
      alert("Failed to delete Kittynode.");
      console.error(error);
    }
  }
</script>

<p>Connect your mobile device:</p>
<button onclick={connectMobile}>Connect mobile</button>
<br>
<br>

<p>Delete all Kittynode data:</p>
<button onclick={deleteKittynode}>Delete data</button>
