<script>
    let { name, value = $bindable() } = $props();
    const uid = $props.id();
</script>

<div class="pane-container">
    <div><label for="{uid}-xml">{name}</label></div>
    <div>
        <textarea {name} id="{uid}-xml" cols="50" rows="30" bind:value></textarea>
    </div>
</div>

<style>
label {
    font-weight: bold;
}

textarea {
    margin-left: 10px;
    margin-right: 10px;
}
</style>
