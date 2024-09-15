<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let runningNodes = 0;

    async function checkRunningNodes() {
        runningNodes = await invoke("check_running_nodes");
    }

    checkRunningNodes();
</script>

<h2>Dashboard</h2>

{#if runningNodes === 0}
    <p>You have no running nodes :'(</p>
    <p>Install one from the package registry!</p>
{:else}
    <p>Running nodes: {runningNodes}</p>
    <p>This is a feature to test CI :)</p>
{/if}
