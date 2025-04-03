<script lang="ts">
    import * as xdiffer from 'libxdiffer';
    import DiffNode from './DiffNode.svelte';

    interface Props {
        tree?: xdiffer.DiffTree;
        set_range1: (range: xdiffer.Range | undefined) => void;
        set_range2: (range: xdiffer.Range | undefined) => void;
    }
    let {tree, set_range1, set_range2}: Props = $props();
</script>

{#if !tree}
<div></div>
{:else if tree.kind() === xdiffer.DiffTreeKind.Same}
<div>No difference</div>
{:else if tree.kind() === xdiffer.DiffTreeKind.TotalDiff}
<div>Root nodes are different</div>
{:else}
<DiffNode value={tree.root()!} set_range1={set_range1} set_range2={set_range2} />
{/if}
