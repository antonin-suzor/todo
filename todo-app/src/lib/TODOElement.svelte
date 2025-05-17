<script lang="ts">
    import type { TODOElementData } from '$lib/types.ts';

    let { data, updateElementData, deleteElement }: {
        data: TODOElementData,
        updateElementData: (elementData: TODOElementData) => Promise<boolean>,
        deleteElement: (elementData: TODOElementData) => void,
    } = $props();

    let doneCheckbox: HTMLInputElement;
    /* Asks the parent to change the "done" value. If not, will revert the changes in the UI. */
    async function changeDone() {
        if (!await updateElementData({...data, done: !data.done})) {
            doneCheckbox.checked = data.done;
        }
    }
</script>

<div style="border: solid 1px grey">
    <p>{data.title}</p>
    <label><input type="checkbox" checked={data.done} onchange={changeDone} bind:this={doneCheckbox}>Done ?</label>
    <p>{data.description}</p>
    <button onclick={() => deleteElement(data)}>Delete</button>
</div>
