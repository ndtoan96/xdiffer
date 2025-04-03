<script lang="ts">
    import * as xdiffer from "libxdiffer";
    import Self from "./DiffNode.svelte";

    interface Props {
        value: xdiffer.DiffNode;
        set_range1: (range: xdiffer.Range | undefined) => void;
        set_range2: (range: xdiffer.Range | undefined) => void;
    }

    let { value, set_range1, set_range2 }: Props = $props();
    const nodeKindMap = {
        [xdiffer.DiffNodeKind.AddedNode]: "added_node",
        [xdiffer.DiffNodeKind.AddedSubNode]: "added_subnode",
        [xdiffer.DiffNodeKind.DeletedNode]: "deleted_node",
        [xdiffer.DiffNodeKind.DeletedSubNode]: "deleted_subnode",
        [xdiffer.DiffNodeKind.UpdatedNode]: "updated_node",
        [xdiffer.DiffNodeKind.NoDiff]: "nodiff",
    };
</script>

<div class="tree">
    <div class="tree-node">
        <span class={nodeKindMap[value.kind()]}><b>{value.name()}</b>
        {#if value.range1() || value.range2()}
        <button onclick={() => {
            set_range1(value.range1());
            set_range2(value.range2());
        }}>Locate</button>
        {/if}
        {#if value.kind() === xdiffer.DiffNodeKind.UpdatedNode || value.kind() === xdiffer.DiffNodeKind.DeletedNode || value.kind() === xdiffer.DiffNodeKind.AddedNode}
        <button>Apply</button>
        {/if}
        <!-- <button>Apply</button></span> -->
        <ul>
            {#each value.children() as child (child)}
                <li>
                    <Self value={child} set_range1={set_range1} set_range2={set_range2} />
                </li>
            {/each}
        </ul>
    </div>
</div>

<style>
    /* Tree container styling */
    .tree {
        font-family: Arial, sans-serif;
        font-size: 14px;
        line-height: 1.5;
    }

    /* Tree node styling */
    .tree-node {
        margin-left: 20px;
    }

    /* Styling for the unordered list to create tree branches */
    .tree-node ul {
        list-style-type: none;
        padding-left: 20px;
        border-left: 1px solid #ccc;
        margin: 5px 0;
    }

    /* Styling for each list item */
    .tree-node li {
        margin: 5px 0;
        padding-left: 10px;
        position: relative;
    }

    /* Connector line for child nodes */
    .tree-node li::before {
        content: '';
        position: absolute;
        left: -10px;
        top: 10px;
        width: 10px;
        height: 1px;
        background-color: #ccc;
    }

    /* Styling for different node types */
    .added_node, .added_subnode {
        color: green;
        font-weight: bold;
    }

    .deleted_node, .deleted_subnode {
        color: red;
        font-weight: bold;
    }

    .updated_node {
        background-color: yellow;
        padding: 2px 4px;
        border-radius: 4px;
    }

    .nodiff {
        color: gray;
        font-style: italic;
    }
</style>