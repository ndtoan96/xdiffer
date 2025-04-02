<script lang="ts">
    import * as xdiffer from "libxdiffer";
    import Self from "./DiffNode.svelte";

    interface Props {
        value: xdiffer.DiffNode;
    }

    let { value }: Props = $props();
    const nodeKindMap = {
        [xdiffer.DiffNodeKind.AddedNode]: "added_node",
        [xdiffer.DiffNodeKind.AddedSubNode]: "added_subnode",
        [xdiffer.DiffNodeKind.DeletedNode]: "deleted_node",
        [xdiffer.DiffNodeKind.DeletedSubNode]: "deleted_subnode",
        [xdiffer.DiffNodeKind.UpdatedNode]: "updated_node",
        [xdiffer.DiffNodeKind.NoDiff]: "nodiff",
    };

</script>

<span class={nodeKindMap[value.kind()]}><b>{value.name()}</b></span>
<ul>
    {#each value.children() as child (child)}
        <li><Self value={child} /></li>
    {/each}
</ul>

<style>
    .added_node,.added_subnode {
        color: green;
    }
    .deleted_node,.deleted_subnode {
        color: red;
    }
    .updated_node {
        background-color: yellow;
    }
</style>