<script lang="ts">
import { page } from "$app/stores";
import { House, HeartPulse, Settings } from "lucide-svelte";
</script>

<nav>
  <div class="flex h-5 items-center justify-between text-sm font-semibold">
    <div class="flex-grow flex justify-center">
      <a class="p-4" href="/" aria-current={$page.url.pathname === "/"}>
        <House strokeWidth={$page.url.pathname === "/" ? 2.25 : 2} />
      </a>
    </div>
    <div class="flex-grow flex justify-center">
      <a
        class="p-4"
        href="/system-info"
        aria-current={$page.url.pathname === "/system-info"}
      >
        <HeartPulse
          strokeWidth={$page.url.pathname === "/system-info" ? 2.25 : 2}
        />
      </a>
    </div>
    <div class="flex-grow flex justify-center">
      <a
        class="p-4"
        href="/settings"
        aria-current={$page.url.pathname === "/settings"}
      >
        <Settings strokeWidth={$page.url.pathname === "/settings" ? 2.25 : 2} />
      </a>
    </div>
  </div>
</nav>

<style>
  a[aria-current="true"] {
    color: #7d3fc1;
  }
</style>
