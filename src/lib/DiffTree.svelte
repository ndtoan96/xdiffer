<script lang="ts">
    import * as xdiffer from 'libxdiffer';
    import DiffNode from './DiffNode.svelte';

    interface Props {
        tree?: xdiffer.DiffTree;
        set_range1: (range: xdiffer.Range | undefined) => void;
        set_range2: (range: xdiffer.Range | undefined) => void;
        id: string;
    }
    let {tree, set_range1, set_range2, id}: Props = $props();
</script>

{#if !tree}
<div id={id}></div>
{:else if tree.kind() === xdiffer.DiffTreeKind.Same}
<div id={id}>No difference</div>
{:else if tree.kind() === xdiffer.DiffTreeKind.TotalDiff}
<div id={id}>Root nodes are different</div>
{:else}
<div id={id}>
    <DiffNode value={tree.root()!} set_range1={set_range1} set_range2={set_range2} />
</div>
{/if}
