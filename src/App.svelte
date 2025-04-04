<script lang="ts">
  import XmlTextPane from "./lib/XmlTextPane.svelte";
  import DiffTree from "./lib/DiffTree.svelte";
  import * as xdiffer from "libxdiffer";
  import {
    appliedEdits,
    STATE_COMPARE,
    STATE_EDIT,
    STATE_PREVIEW,
  } from "./lib/shared.svelte";
  import { getCurrentDiffNode, setCurrentDiffNode } from "./lib/shared.svelte";

  let xml1 = $state("");
  let xml2 = $state("");
  let mergedXml = $state("");
  let range1: xdiffer.Range | undefined = $state(undefined);
  let range2: xdiffer.Range | undefined = $state(undefined);
  let stateMachine = $state(STATE_EDIT);
  let diffTree: xdiffer.DiffTree | undefined = $state(undefined);
  let diffNodeIndex = $derived.by(() => {
    const nodes = document.getElementsByClassName("diff-node");
    if (getCurrentDiffNode()) {
      for (let i = 0; i < nodes.length; i++) {
        if (nodes[i] === getCurrentDiffNode()) {
          return i + 1;
        }
      }
    }
    return "?";
  });
  let totalDiffNodes = $derived.by(() =>
    diffTree ? diffTree.diff_count() : 0,
  );

  function onBackBtnClick() {
    if (stateMachine === STATE_COMPARE) {
      stateMachine = STATE_EDIT;
    } else if (stateMachine === STATE_PREVIEW) {
      stateMachine = STATE_COMPARE;
    }
  }

  function onCompareBtnClick() {
    try {
      diffTree = xdiffer.build_diff_tree(xml1, xml2);
      stateMachine = STATE_COMPARE;
    } catch (e) {
      alert(e);
    }
  }

  function onPreviewBtnClick() {
    mergedXml = xdiffer.apply_changes(xml1, xml2, [
      ...appliedEdits.values().map((c) => c.copy()), // creating copy to avoid moving the original value
    ]);
    stateMachine = STATE_PREVIEW;
  }

  function onSaveBtnClick() {
    const link = document.createElement("a");
    const file = new Blob([mergedXml], { type: "application/xml" });
    link.href = URL.createObjectURL(file);
    link.download = "merged.xml";
    link.click();
    URL.revokeObjectURL(link.href);
  }

  function onPrevDiffBtnClick() {
    const nodes = document.getElementsByClassName("diff-node");
    if (getCurrentDiffNode()) {
      for (let i = 0; i < nodes.length; i++) {
        if (nodes[i] === getCurrentDiffNode()) {
          setCurrentDiffNode(nodes[(i + nodes.length - 1) % nodes.length]);
          getCurrentDiffNode()?.scrollIntoView({
            behavior: "instant",
            block: "center",
            inline: "center",
          });
          return;
        }
      }
    }
    setCurrentDiffNode(nodes[nodes.length - 1]);
    nodes[nodes.length - 1].scrollIntoView({
      behavior: "instant",
      block: "center",
      inline: "center",
    });
  }

  function onNextDiffBtnClick() {
    const nodes = document.getElementsByClassName("diff-node");
    if (getCurrentDiffNode()) {
      for (let i = 0; i < nodes.length; i++) {
        if (nodes[i] === getCurrentDiffNode()) {
          setCurrentDiffNode(nodes[(i + 1) % nodes.length]);
          getCurrentDiffNode()?.scrollIntoView({
            behavior: "instant",
            block: "center",
            inline: "center",
          });
          return;
        }
      }
    }
    setCurrentDiffNode(nodes[0]);
    nodes[0].scrollIntoView({
      behavior: "instant",
      block: "center",
      inline: "center",
    });
  }
</script>

<main>
  <div class="btn-container">
    {#if stateMachine === STATE_EDIT}
      <button class="btn compare-btn" onclick={onCompareBtnClick}
        >Compare</button
      >
    {:else if stateMachine === STATE_COMPARE}
      <button class="btn back-btn" onclick={onBackBtnClick}>Back</button>
      <button class="btn preview-btn" onclick={onPreviewBtnClick}
        >Preview</button
      >
      <button class="btn prevdiff-btn" onclick={onPrevDiffBtnClick}
        >Previous</button
      >
      <button class="btn nextdiff-btn" onclick={onNextDiffBtnClick}>Next</button
      >
      <div class="diff-counter">
        <span>{diffNodeIndex}/{totalDiffNodes}</span>
      </div>
    {:else}
      <button class="btn back-btn" onclick={onBackBtnClick}>Back</button>
      <button class="btn save-btn" onclick={onSaveBtnClick}>Save</button>
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
    {:else if stateMachine === STATE_PREVIEW}
      <div class="preview-container">
        <div><label for="preview"><b>Preview</b></label></div>
        <textarea class="preview" name="preview" id="preview" bind:value={mergedXml}></textarea>
      </div>
    {/if}
  </div>
</main>

<style>
  .central-container {
    display: flex;
    height: 80vh;
  }

  div.btn-container {
    display: flex;
  }

  .xml1 {
    height: 100%;
    margin-right: 5px;
  }

  .xml2 {
    height: 100%;
    margin-left: 5px;
  }

  .diff-tree,
  .preview-container {
    width: 30vw;
    height: 100%;
    margin-left: 5px;
  }

  .preview {
    width: 100%;
    height: 100%;
    overflow: scroll;
    white-space: nowrap;
  }

  .btn-container {
    margin-bottom: 20px;
  }

  .btn {
    margin: 5px;
    padding: 10px 20px;
    font-size: 16px;
    font-weight: 500;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.3s ease;
    background-color: #4caf50;
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
    background-color: #2196f3;
  }

  .preview-btn {
    background-color: #4caf50;
  }

  .back-btn {
    background-color: #757575;
  }

  .prevdiff-btn {
    margin-left: auto;
  }

  .prevdiff-btn,
  .nextdiff-btn {
    background-color: #63a2d3;
  }

  .save-btn {
    background-color: #2196f3;
  }

  .diff-counter {
    margin-top: auto;
    margin-bottom: auto;
  }
</style>
