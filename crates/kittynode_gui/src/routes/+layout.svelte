<script lang="ts">
  import "@picocss/pico/css/pico.min.css";
  import { page } from "$app/stores";
  import logo from "$lib/assets/logo.jpg";
</script>

<main class="container">
  <nav>
    <ul>
      <li>
        <img src={logo} alt="Kittynode Logo" class="logo" />
      </li>
    </ul>
    <ul>
      <li>
        <a href="/" aria-current={$page.url.pathname === "/"}>Home</a>
      </li>
      <li>
        <a
          href="/package-store"
          aria-current={$page.url.pathname === "/package-store"}
          >Package store</a
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
  <hr />
  <slot />
</main>

<style>
  main {
    padding: 1rem;
  }

  .logo {
    width: 50px;
    height: 50px;
    border-radius: 50%;
  }
</style>
