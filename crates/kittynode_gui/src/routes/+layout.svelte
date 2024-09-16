<script lang="ts">
  import "@picocss/pico/css/pico.min.css";
  import { page } from "$app/stores";
</script>

<header class="container">
  <nav>
    <ul>
      <li>
        <strong>Kittynode</strong>
      </li>
    </ul>
    <ul>
      <li>
        <a href="/" aria-current={$page.url.pathname === "/"}>Home</a>
      </li>
      <li>
        <a href="/packages" aria-current={$page.url.pathname === "/packages"}
          >Packages</a
        >
      </li>
      <li>
        <a href="/ports" aria-current={$page.url.pathname === "/ports"}>Ports</a
        >
      </li>
      <li>
        <a href="/settings" aria-current={$page.url.pathname === "/settings"}
          >Settings</a
        >
      </li>
    </ul>
  </nav>
</header>

<main class="container">
  <slot />
</main>
