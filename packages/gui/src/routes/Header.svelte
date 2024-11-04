<script>
import { page } from "$app/stores";
</script>

<header class="container">
  <nav>
    <ul>
      <li>
        <a href="/" aria-current={$page.url.pathname === "/"}>Home</a>
      </li>
      <li>
        <a href="/system-info" aria-current={$page.url.pathname === "/system-info"}>System Info</a>
      </li>
      <li>
        <a href="/settings" aria-current={$page.url.pathname === "/settings"}>Settings</a>
      </li>
    </ul>
  </nav>
</header>

<style>
a[aria-current="true"] {
  font-weight: bold;
}
</style>
