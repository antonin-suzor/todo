<script lang="ts">
    import type { TODOElementData } from '$lib/types.ts';

    let { title, done, description }: TODOElementData = $props();
</script>

<div>
    <p>{title}</p>
    <label><input type="checkbox" checked={done}>Done ?</label>
    <p>{description}</p>
</div>
