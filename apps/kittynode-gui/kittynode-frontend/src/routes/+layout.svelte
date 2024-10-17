<script lang="ts">
  import "@picocss/pico/css/pico.purple.min.css";
  import { page } from "$app/stores";
</script>

<header class="container">
  <nav>
    <ul>
      <li>
        <a href="/">
          <img class="logo" alt="Kittynode Logo" />
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

<style>
  /* Default logo for light mode */
  img.logo {
    height: 1.5em;
    vertical-align: middle;
    content: url('/images/kittynode-light.png');
  }

  /* Change logo in dark mode */
  @media (prefers-color-scheme: dark) {
    img.logo {
      content: url('/images/kittynode-dark.png');
    }
  }
</style>
