<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let isDockerRunning: null | boolean = null;

  async function checkDocker() {
    try {
      isDockerRunning = await invoke("is_docker_running");
    } catch (error) {
      console.error("Failed to check Docker", error);
    }
  }

  onMount(() => {
    checkDocker();
  });
</script>

<p>Docker is <mark>{isDockerRunning === null ? "" : (isDockerRunning ? "running" : "not running")}</mark></p>
<button on:click={checkDocker}>Check Docker</button>
