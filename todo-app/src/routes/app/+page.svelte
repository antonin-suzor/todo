<script lang="ts">
    import type { TODOElementData } from '$lib/types.ts';
    import TODOElement from '$lib/TODOElement.svelte'

    let elements: TODOElementData[] = [
        {
            title: "TODO element",
            done: false,
            description: "Element description"
        },
        {
            title: "DONE element",
            done: true,
            description: "Element description"
        }
    ];
</script>

<h1>TODO app</h1>
<a href="/">Go to the home page</a>

<h3>TODO</h3>
<TODOElement {...elements[0]} />

<h3>DONE</h3>
<TODOElement {...elements[1]} />