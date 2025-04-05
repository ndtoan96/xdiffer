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
    let btnText = $derived.by(inferNodeBtnText);

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

    function inferNodeBtnText(): string {
        if (value.kind() === xdiffer.DiffNodeKind.AddedNode) {
            if (applied) {
                return "XML2";
            } else {
                return "None";
            }
        } else if (value.kind() === xdiffer.DiffNodeKind.DeletedNode) {
            if (applied) {
                return "None";
            } else {
                return "XML1";
            }
        } else {
            if (applied) {
                return "XML2";
            } else {
                return "XML1";
            }
        }
    }
</script>

<div class="node">
    <div class="hline"></div>
    {#if value.kind() === xdiffer.DiffNodeKind.UpdatedNode || value.kind() === xdiffer.DiffNodeKind.DeletedNode || value.kind() === xdiffer.DiffNodeKind.AddedNode}
        <a
            href="/"
            class={{
                [nodeKindMap[value.kind()]]: true,
                "diff-node": true,
                "attribute-node": value.is_attribute(),
                "element-node": value.is_element(),
                "text-node": value.is_text(),
                faded: btnText == "None",
            }}
            onclick={onChangedNodeClick}>{value.name()}</a
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
            }}>{btnText}</button
        >
    {:else}
        <span
            class={{
                [nodeKindMap[value.kind()]]: true,
                "attribute-node": value.is_attribute(),
                "element-node": value.is_element(),
                "text-node": value.is_text(),
            }}>{value.name()}</span
        >
    {/if}
</div>
<!-- <button>Apply</button></span> -->
<ul class={{ faded: btnText == "None" }}>
    {#each value.children() as child (child)}
        <li>
            <Self value={child} {set_range1} {set_range2} />
        </li>
    {/each}
</ul>

<style>
    a,
    span {
        white-space: nowrap;
    }

    ul {
        list-style: none;
        margin-top: 0px;
        padding-top: 0.3rem;
    }

    li {
        width: max-content;
        border-left: 1px solid rgb(172, 172, 236);
    }

    .node {
        display: flex;
    }

    .node button {
        min-width: 3rem;
        margin-left: 0.5rem;
    }

    .hline {
        width: 1rem;
        height: 1px;
        margin-top: 0.5rem;
        background-color: rgb(172, 172, 236);
    }

    /* Styling for different node types */
    .element-node {
        font-weight: bold;
        color: #3a2f73;
    }

    .attribute-node {
        font-weight: 200;
        color: #0eca1a;
    }

    .text-node {
        font-style: italic;
    }

    .added-node,
    .added-subnode {
        background-color: yellow;
    }

    .deleted-node,
    .deleted-subnode {
        background-color: yellow;
    }

    .updated-node {
        background-color: yellow;
    }

    .faded {
        opacity: 0.2;
    }
</style>
