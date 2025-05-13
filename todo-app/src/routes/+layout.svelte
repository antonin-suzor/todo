<script lang="ts">
    let { children, data } = $props();
</script>

<nav>
    <h1>TODO app</h1>
    <a href="/">Home</a>
    <a href="/app">App</a>
    {#if data.user}
        <p>Logged in as: {data.user.name}</p>
    {:else}
        <p>You are not logged in.</p>
    {/if}
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