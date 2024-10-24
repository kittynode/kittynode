<script lang="ts">
import "@picocss/pico/css/pico.purple.min.css";
import { onMount } from "svelte";
import { win } from "../stores/window.svelte.ts";
import { initializedStore } from "../stores/initialized.svelte";
import Welcome from "./Welcome.svelte";
import Header from "./Header.svelte";

const { children } = $props();

onMount(async () => {
  console.log("initializedStore.initialized", initializedStore.initialized);
  await win.show();
});
</script>

{#if !initializedStore.initialized}
<Welcome />
{:else}
<Header />
<main class="container" style="margin-top: 1rem;">
  {@render children()}
</main>
{/if}
