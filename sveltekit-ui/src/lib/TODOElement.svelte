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

    let changingTitle = $state(false);
    let titleInput: HTMLInputElement;
    function startChangingTitle() {
        changingTitle = true;
    }
    /* Asks the parent to change the "title" value. If successful, will turn the UI back to normal. */
    async function confirmChangeTitle() {
        if (data.title === titleInput.value || await updateElementData({...data, title: titleInput.value})) {
            changingTitle = false;
        }
    }
    let resetTitleInput: HTMLButtonElement;
    function resetChangeTitle() {
        titleInput.value = data.title;
        changingTitle = false;
    }
    /* Resets or confirms the changes to the title if the user presses Escape or Enter. */
    function titleInputEscapeOrEnter(e: KeyboardEvent) {
        if (e.key === 'Escape') {
            resetChangeTitle();
        }
        else if (e.key === 'Enter') {
            confirmChangeTitle();
        }
    }
</script>

<div style="border: solid 1px grey">
    {#if !changingTitle}
        <p onclick={startChangingTitle}>{data.title}</p>
        <button onclick={startChangingTitle}>Change</button>
    {:else}
        <input type="text" value={data.title} bind:this={titleInput} autofocus
               onfocusout={(e) => {if (e.relatedTarget !== resetTitleInput) confirmChangeTitle()}}
               onkeydown={titleInputEscapeOrEnter}>
        <button onclick={resetChangeTitle} bind:this={resetTitleInput}>Reset</button>
    {/if}
    <label><input type="checkbox" checked={data.done} onchange={changeDone} bind:this={doneCheckbox}>Done ?</label>
    <p>{data.description}</p>
    <button onclick={() => deleteElement(data)}>Delete</button>
</div>
