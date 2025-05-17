<script lang="ts">
    import type { TODOElementData } from '$lib/types.ts';

    let { data, updateElementData, deleteElement }: {
        data: TODOElementData,
        updateElementData: (elementData: TODOElementData) => void,
        deleteElement: (elementData: TODOElementData) => void,
    } = $props();

    function changeDone() {
        updateElementData({...data, done: !data.done});
    }
</script>

<div style="border: solid 1px grey">
    <p>{data.title}</p>
    <label><input type="checkbox" checked={data.done} onchange={changeDone}>Done ?</label>
    <p>{data.description}</p>
    <button onclick={() => deleteElement(data)}>Delete</button>
</div>
