import * as xdiffer from 'libxdiffer';
import { SvelteMap } from 'svelte/reactivity';

export const STATE_EDIT = 0;
export const STATE_COMPARE = 1;
export const STATE_PREVIEW = 2;
export let appliedEdits: SvelteMap<string, xdiffer.Change> = $state(new SvelteMap());
let currentDiffNode: Element | undefined = $state(undefined);

export function setCurrentDiffNode(node: Element | undefined) {
    currentDiffNode = node;
}

export function getCurrentDiffNode(): Element | undefined {
    return currentDiffNode;
}