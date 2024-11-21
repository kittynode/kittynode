<script lang="ts">
import type { Package } from "$lib/types";
import { onMount } from "svelte";

let blah = $props();


onMount(() => {
  console.log("name", blah);
});

</script>

<a href="/" class="text-primary font-medium underline underline-offset-4">← Back home</a>
<br />
<br />
<p>Package stuff here</p>

<p>here is my name: { blah }</p>
