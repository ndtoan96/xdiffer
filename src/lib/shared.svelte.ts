import * as xdiffer from 'libxdiffer';

export const STATE_EDIT = 0;
export const STATE_COMPARE = 1;
export const STATE_PREVIEW = 2;
export const appliedEdits: Map<string, xdiffer.Change> = new Map();