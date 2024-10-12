<script lang="ts">
  import "@picocss/pico/css/pico.purple.min.css";
  import { page } from "$app/stores";
  import logo from '$lib/images/kittynode-dark.png';
</script>

<header class="container">
  <nav>
    <ul>
      <li>
        <a href="/">
          <img src={logo} alt="Kittynode Logo" style="height: 1.5em; vertical-align: middle;" />
        </a>
      </li>
    </ul>
    <ul>
      <li>
        <a href="/" aria-current={$page.url.pathname === "/"}>Home</a>
      </li>
      <li>
        <a href="/packages" aria-current={$page.url.pathname === "/packages"}>Packages</a>
      </li>
    </ul>
  </nav>
</header>

<main class="container" style="margin-top: 1rem;">
  <slot />
</main>
