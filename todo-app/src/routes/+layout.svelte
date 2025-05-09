<script lang="ts">
    let { children } = $props();
</script>

<nav>
    <h1>TODO app</h1>
    <a href="/">Home</a>
    <a href="/app">App</a>
</nav>

{@render children()}

<style>
    nav {
        background-image: linear-gradient(to bottom right, darkblue, blue);
        color: lightgrey;
        * {
            color: lightgrey;
        }
    }
</style>