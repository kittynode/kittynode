<script lang="ts">
  import { onMount } from "svelte";
  import { window } from "../stores/window.svelte.ts";
  import { Kaomoji } from "../components";

  onMount(() => {
    window.show();
  });
</script>

<p>Welcome to Kittynode!</p>
<Kaomoji />
