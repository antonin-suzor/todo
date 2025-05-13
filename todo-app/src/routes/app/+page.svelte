<script lang="ts">
    import type { TODOElementData } from '$lib/types.ts';
    import type { appPageServerProps } from './+page.server.ts';
    import TODOElement from '$lib/TODOElement.svelte'

    let { data }: { data: appPageServerProps } = $props();
    let elements: TODOElementData[] = $state(data.elementsData);

    let addTitle = $state('');
    let addDone = $state(false);
    let addDescription = $state('');
    function addElement() {
        elements.push({
            id: Date.now(),
            title: addTitle,
            done: addDone,
            description: addDescription,
        });
        addTitle = '';
        addDone = false;
        addDescription = '';
    }

    function updateElementData(elementData: TODOElementData) {
        let i = elements.findIndex(e => e.id === elementData.id);
        elements[i] = elementData;
    }
</script>

<h3>Add</h3>
<form onsubmit={addElement}>
    <label>Title: <input type="text" bind:value={addTitle}></label>
    <label><input type="checkbox" bind:checked={addDone}>Done ?</label>
    <label>Description: <input type="text" bind:value={addDescription}></label>
    <input type="submit" value="Add">
</form>

<h3>TODO</h3>
{#each elements.filter(e => !e.done) as elem (elem.id)}
    <TODOElement data={elem} updateElementData={updateElementData} />
{/each}

<h3>DONE</h3>
{#each elements.filter(e => e.done) as elem (elem.id)}
    <TODOElement data={elem} updateElementData={updateElementData} />
{/each}