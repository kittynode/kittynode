<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    async function installNode() {
        await invoke("install_node");
    }

    async function checkDockerVersion() {
        await invoke("check_docker_version");
    }
</script>

<h2>Package registry</h2>
<p>
    Welcome to the package registry! Here you can find available packages to
    install.
</p>

<p>Package list:</p>
<p>Taiko</p>
<button on:click={installNode}>Install Node</button>
<button on:click={checkDockerVersion}>Check Docker Version</button>
