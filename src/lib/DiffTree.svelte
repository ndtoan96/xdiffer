<script lang="ts">
    import * as xdiffer from 'libxdiffer';
    import DiffNode from './DiffNode.svelte';

    interface Props {
        tree?: xdiffer.DiffTree;
    }
    let {tree}: Props = $props();
</script>

{#if !tree}
<div></div>
{:else if tree.kind() === xdiffer.DiffTreeKind.Same}
<div>No difference</div>
{:else if tree.kind() === xdiffer.DiffTreeKind.TotalDiff}
<div>Root nodes are different</div>
{:else}
<DiffNode value={tree.root()!} />
{/if}
