<script lang="ts">
    import TODOElement from '$lib/TODOElement.svelte'
</script>

<h1>TODO app</h1>
<a href="/">Go to the home page</a>

<h3>TODO</h3>
<TODOElement title={'TODO element'} description={'Element description'}/>

<h3>DONE</h3>
<TODOElement title={'DONE element'} done={true} description={'Element description'}/>