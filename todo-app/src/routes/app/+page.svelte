<script lang="ts">
    import type { TODOElementData } from '$lib/types.ts';
    import TODOElement from '$lib/TODOElement.svelte'

    let { data } = $props();
    let elements: TODOElementData[] = $state(data.elementsData);

    let addTitle = $state('');
    let addDone = $state(false);
    let addDescription = $state('');
    async function addElement() {
        try {
            let res = await fetch('/api/todos', {
                method: 'POST',
                body: JSON.stringify({
                    title: addTitle,
                    done: addDone,
                    description: addDescription,
                }),
                headers: {
                    'Content-Type': 'application/json',
                }
            });
            if (!res.ok) {
                throw new Error(res.statusText);
            }
            let newTodo: TODOElementData = await res.json();
            elements.push(newTodo);
            addTitle = '';
            addDone = false;
            addDescription = '';
        } catch (e) {
            console.error(e);
            alert('Something went wrong when sending new TODO to the server.');
        }
    }

    async function updateElementData(elementData: TODOElementData) {
        try {
            // do not await yet, simply send the request
            let resPromise = fetch(`/api/todos/${elementData.id}`, {
                method: 'PATCH',
                body: JSON.stringify(elementData),
                headers: {
                    'Content-Type': 'application/json',
                }
            });
            // then, iterate through the array
            let i = elements.findIndex(e => e.id === elementData.id);
            // then, you may await
            let res = await resPromise;
            if (!res.ok) {
                throw new Error(res.statusText);
            }
            elements[i] = await res.json();
        } catch (e) {
            console.error(e);
            alert('Something went wrong when modifying the TODO on the server.');
        }
    }

    async function deleteElement(elementData: TODOElementData) {
        try {
            let res = await fetch(`/api/todos/${elementData.id}`, { method: 'DELETE' });
            if (!res.ok) {
                throw new Error(res.statusText);
            }
            elements = elements.filter(e => e.id !== elementData.id)
        } catch (e) {
            console.error(e);
            alert('Something went wrong when modifying the TODO on the server.');
        }
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
    <TODOElement data={elem} updateElementData={updateElementData} deleteElement={deleteElement} />
{/each}

<h3>DONE</h3>
{#each elements.filter(e => e.done) as elem (elem.id)}
    <TODOElement data={elem} updateElementData={updateElementData} deleteElement={deleteElement} />
{/each}