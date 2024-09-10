<script lang="ts">
    import { page } from "$app/stores";
</script>

<div class="app">
    <header>
        <h1>kittynode</h1>
        <nav>
            <a href="/" class:active={$page.url.pathname === "/"}>Dashboard</a>
            <a
                href="/package-registry"
                class:active={$page.url.pathname === "/package-registry"}
                >Package registry</a
            >
            <a
                href="/port-manager"
                class:active={$page.url.pathname === "/port-manager"}
                >Port manager</a
            >
            <a
                href="/security"
                class:active={$page.url.pathname === "/security"}>Security</a
            >
        </nav>
    </header>

    <main>
        <slot />
    </main>
</div>

<style>
    .app {
        display: flex;
        flex-direction: column;
        min-height: 100vh;
    }
    header {
        padding: 1rem;
        background: #f0f0f0;
    }
    nav {
        display: flex;
        gap: 1rem;
    }
    main {
        flex: 1;
        padding: 1rem;
    }
    .active {
        font-weight: bold;
    }
</style>
