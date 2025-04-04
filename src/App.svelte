<script lang="ts">
  import XmlTextPane from "./lib/XmlTextPane.svelte";
  import DiffTree from "./lib/DiffTree.svelte";
  import * as xdiffer from "libxdiffer";
  import { appliedEdits, STATE_COMPARE, STATE_EDIT } from "./lib/shared.svelte";

  let xml1 = $state("");
  let xml2 = $state("");
  let range1: xdiffer.Range | undefined = $state(undefined);
  let range2: xdiffer.Range | undefined = $state(undefined);
  let stateMachine = $state(STATE_EDIT);
  let diffTree: xdiffer.DiffTree | undefined = $state(undefined);

  function onCompareBtnClick() {
    try {
      diffTree = xdiffer.build_diff_tree(xml1, xml2);
    } catch (e) {
      alert(e);
    }
    stateMachine = STATE_COMPARE;
  }

  function onMergeBtnClick() {
    const merged_xml = xdiffer.apply_changes(xml1, xml2, [
      ...appliedEdits.values().map((c) => c.copy()), // creating copy to avoid moving the original value
    ]);
    const link = document.createElement("a");
    const file = new Blob([merged_xml], { type: "application/xml" });
    link.href = URL.createObjectURL(file);
    link.download = "merged.xml";
    link.click();
    URL.revokeObjectURL(link.href);
  }
</script>

<main>
  <div class="btn-container">
    {#if stateMachine === STATE_EDIT}
      <button class="btn compare-btn" onclick={onCompareBtnClick}
        >Compare</button
      >
    {:else if stateMachine === STATE_COMPARE}
      <button class="btn back-btn" onclick={() => (stateMachine = STATE_EDIT)}
        >Back</button
      >
      <button class="btn merge-btn" onclick={onMergeBtnClick}>Merge</button>
    {/if}
  </div>
  <div class="central-container">
    <div class="xml1">
      <XmlTextPane
        name="XML 1"
        bind:value={xml1}
        {stateMachine}
        range={range1}
      />
    </div>
    <div class="xml2">
      <XmlTextPane
        name="XML 2"
        bind:value={xml2}
        {stateMachine}
        range={range2}
      />
    </div>
    {#if stateMachine === STATE_COMPARE}
      <div class="diff-tree">
        <label for="diff-tree"><b>Diff Tree</b></label>
        <DiffTree
          tree={diffTree}
          set_range1={(r) => (range1 = r)}
          set_range2={(r) => (range2 = r)}
          id="diff-tree"
        />
      </div>
    {/if}
  </div>
</main>

<style>
  .central-container {
    display: flex;
  }

  div.btn-container {
    text-align: center;
  }

  .xml1 {
    margin-right: 5px;
  }

  .xml2 {
    margin-left: 5px;
  }

  .diff-tree {
    width: 30vw;
    height: 80vh;
    overflow: scroll;
  }

  .btn-container {
    margin-bottom: 20px;
  }

  .btn {
    padding: 10px 20px;
    font-size: 16px;
    font-weight: 500;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.3s ease;
    background-color: #4CAF50;
    color: white;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  }

  .btn:active {
    transform: translateY(0);
  }

  .compare-btn {
    background-color: #2196F3;
  }

  .merge-btn {
    background-color: #4CAF50;
  }

  .back-btn {
    background-color: #757575;
  }
</style>
