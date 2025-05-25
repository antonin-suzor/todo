<script lang="ts">
    import { onMount } from 'svelte';

    let actixCount: number | undefined = $state();
    let actixMessage: string | undefined = $state();
    onMount(async () => {
        let res = await fetch('http://localhost:8080/count');
        actixCount = (await res.json()).count;
        if (!res.ok) {
            console.error(res.statusText);
        }

        res = await fetch('http://localhost:8080/');
        if (!res.ok) {
            console.error(res.statusText);
        }
        actixMessage = await res.text();
    })
</script>

<h3>Start small, then expand.</h3>
{#if actixCount}
    <p>This page has called the Actix back-end {actixCount} times.</p>
{/if}
{#if actixMessage}
    <p>Message from the Actix back-end:</p>
    <code>{actixMessage}</code>
{/if}
