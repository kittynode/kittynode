<script lang="ts">
  import { page } from "$app/stores";
  import { Separator } from "$lib/components/ui/separator";
  import { House, HeartPulse, Settings } from "lucide-svelte";
</script>

<nav>
  <Separator class="my-4" />
  <div
    class="flex h-5 items-center justify-center space-x-4 text-sm font-semibold"
  >
    <a href="/" aria-current={$page.url.pathname === "/"}><House /></a>
    <Separator orientation="vertical" />
    <a href="/system-info" aria-current={$page.url.pathname === "/system-info"}
      ><HeartPulse /></a
    >
    <Separator orientation="vertical" />
    <a href="/settings" aria-current={$page.url.pathname === "/settings"}
      ><Settings /></a
    >
  </div>
</nav>

<style>
  a[aria-current="true"] {
    color: #7d3fc1; /* Adjust this shade of purple as needed */
  }
</style>
