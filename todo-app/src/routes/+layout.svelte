<script lang="ts">
    let { children, data } = $props();

    function logInAsUserId(userId: number) {
        fetch(`/api/fake-auth/as/${userId}`);
    }
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
    <ul>
        Login as:
        <li><button onclick={() => logInAsUserId(1)}>User 1, ABC</button></li>
        <li><button onclick={() => logInAsUserId(2)}>User 2, DEF</button></li>
        <li><button onclick={() => logInAsUserId(0)}>Log out</button></li>
    </ul>
</nav>

{@render children()}

<style>
    nav {
        background-image: linear-gradient(to bottom right, darkblue, blue);
        color: lightgrey;
        * {
            color: lightgrey;
            button {
                color: black;
            }
        }
    }
</style>