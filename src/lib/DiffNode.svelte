<script lang="ts">
    import * as xdiffer from "libxdiffer";
    import Self from "./DiffNode.svelte";
    import { appliedEdits } from "./shared.svelte";

    interface Props {
        value: xdiffer.DiffNode;
        set_range1: (range: xdiffer.Range | undefined) => void;
        set_range2: (range: xdiffer.Range | undefined) => void;
    }

    function deriveKey(v: xdiffer.DiffNode): string {
        return `${value.range1()}|${value.range2()}`;
    } 
    
    let { value, set_range1, set_range2 }: Props = $props();
    let applied = $derived(appliedEdits.has(deriveKey(value)));
    const nodeKindMap = {
        [xdiffer.DiffNodeKind.AddedNode]: "added-node",
        [xdiffer.DiffNodeKind.AddedSubNode]: "added-subnode",
        [xdiffer.DiffNodeKind.DeletedNode]: "deleted-node",
        [xdiffer.DiffNodeKind.DeletedSubNode]: "deleted-subnode",
        [xdiffer.DiffNodeKind.UpdatedNode]: "updated-node",
        [xdiffer.DiffNodeKind.NoDiff]: "nodiff",
    };

    function onChangedNodeClick(e: Event) {
        e.preventDefault();
        set_range1(value.range1());
        set_range2(value.range2());
    }
</script>

<div class="tree">
    <div class="tree-node">
        {#if value.kind() === xdiffer.DiffNodeKind.UpdatedNode || value.kind() === xdiffer.DiffNodeKind.DeletedNode || value.kind() === xdiffer.DiffNodeKind.AddedNode}
            <a
                href="/"
                class={[nodeKindMap[value.kind()], "diff-node"]}
                onclick={onChangedNodeClick}><b>{value.name()}</b></a
            >
            <button
                onclick={() => {
                    const key = deriveKey(value);
                    if (applied) {
                        appliedEdits.delete(key);
                    } else {
                        appliedEdits.set(
                            key,
                            new xdiffer.Change(
                                value.range1(),
                                value.range2(),
                                value.insert_pos(),
                                value.is_attribute(),
                            ),
                        );
                    }
                }}
                >{#if applied}Revert{:else}Apply{/if}</button
            >
        {:else}
            <span class={nodeKindMap[value.kind()]}><b>{value.name()}</b></span>
        {/if}
        <!-- <button>Apply</button></span> -->
        <ul>
            {#each value.children() as child (child)}
                <li>
                    <Self value={child} {set_range1} {set_range2} />
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
        height: 100%;
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
        content: "";
        position: absolute;
        left: -10px;
        top: 10px;
        width: 10px;
        height: 1px;
        background-color: #ccc;
    }

    /* Styling for different node types */
    .added-node,
    .added-subnode {
        color: green;
        font-weight: bold;
    }

    .deleted-node,
    .deleted-subnode {
        color: red;
        font-weight: bold;
    }

    .updated-node {
        background-color: yellow;
        padding: 2px 4px;
        border-radius: 4px;
    }

    .nodiff {
        color: gray;
        font-style: italic;
    }
</style>
