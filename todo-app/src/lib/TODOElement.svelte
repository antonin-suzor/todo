<script lang="ts">
    let {title, done = false, description}: { title: string, done?: boolean, description: string } = $props();
</script>

<div>
    <p>{title}</p>
    <label><input type="checkbox" checked={done}>Done ?</label>
    <p>{description}</p>
</div>
