<script lang="ts">
    import type { TODOElementData } from '$lib/types.ts';
    import TODOElement from '$lib/TODOElement.svelte'

    let elements: TODOElementData[] = $state([
        {
            id: 1,
            title: "TODO element",
            done: false,
            description: "Element description"
        },
        {
            id: 2,
            title: "DONE element",
            done: true,
            description: "Element description"
        }
    ]);

    function updateElementData(elementData: TODOElementData) {
        let i = elements.findIndex(e => e.id === elementData.id);
        elements[i] = elementData;
    }
</script>

<h1>TODO app</h1>
<a href="/">Go to the home page</a>

<h3>TODO</h3>
{#each elements.filter(e => !e.done) as elem (elem.id)}
    <TODOElement data={elem} updateElementData={updateElementData} />
{/each}

<h3>DONE</h3>
{#each elements.filter(e => e.done) as elem (elem.id)}
    <TODOElement data={elem} updateElementData={updateElementData} />
{/each}