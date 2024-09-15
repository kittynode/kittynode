<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  async function installNode() {
    await invoke("install_node");
  }

  async function checkDockerVersion() {
    await invoke("check_docker_version");
  }
</script>

<p>
  Welcome to the package store! Here you can find available packages to install.
</p>
<button on:click={installNode}>Install Node</button>
<button on:click={checkDockerVersion}>Check Docker Version</button>
