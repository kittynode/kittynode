<script lang="ts">
import { invoke } from "@tauri-apps/api/core";

function initKittynode() {
  invoke("init_kittynode");
  window.location.href = "/"; // Navigate to home page after invoking
}
</script>

<main>
  <img class="logo" src="/images/kittynode-light.png" alt="Kittynode Logo">
  <button onclick={initKittynode}>Get Started</button>
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 100vh;
    text-align: center;
  }

  /* Default logo for light mode */
  img.logo {
    width: 300px; /* Adjust logo size */
    margin-bottom: 20px;
    vertical-align: middle;
    content: url("/images/kittynode-light.png");
  }

  /* Change logo in dark mode */
  @media (prefers-color-scheme: dark) {
    img.logo {
    content: url("/images/kittynode-dark.png");
    }
  }
</style>
