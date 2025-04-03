<script lang="ts">
  import XmlTextPane from "./lib/XmlTextPane.svelte";
  import DiffTree from "./lib/DiffTree.svelte";
  import * as xdiffer from "libxdiffer";
  import { appliedEdits } from "./lib/shared.svelte";

  let xml1 = $state("");
  let xml2 = $state("");
  let range1: xdiffer.Range | undefined = $state(undefined);
  let range2: xdiffer.Range | undefined = $state(undefined);
  let editState = $state(true);
  let diffTree: xdiffer.DiffTree | undefined = $state(undefined);

  function onBtnClick() {
    if (editState) {
      editState = false;
      try {
        diffTree = xdiffer.build_diff_tree(xml1, xml2);
      } catch (e) {
        console.error(e);
      }
    } else {
      diffTree = undefined;
      editState = true;
    }
  }
</script>

<main>
  <div class="text-input-section">
    <XmlTextPane
      name="XML 1"
      bind:value={xml1}
      edit={editState}
      range={range1}
    />
    <XmlTextPane
      name="XML 2"
      bind:value={xml2}
      edit={editState}
      range={range2}
    />
  </div>
  <div class="btn-container">
    <button onclick={onBtnClick}
      >{#if editState}Compare{:else}Back{/if}</button
    >
    {#if !editState}<button
        onclick={() => {
          const merged_xml = xdiffer.apply_changes(xml1, xml2, [
            ...appliedEdits.values().map((c) => c.copy()), // creating copy to avoid moving the original value
          ]);
          const link = document.createElement("a");
          const file = new Blob([merged_xml], { type: "application/xml" });
          link.href = URL.createObjectURL(file);
          link.download = "merged.xml";
          link.click();
          URL.revokeObjectURL(link.href);
        }}>Merge</button
      >{/if}
  </div>
  {#if !editState}
    <div>
      <DiffTree
        tree={diffTree}
        set_range1={(r) => (range1 = r)}
        set_range2={(r) => (range2 = r)}
      />
    </div>
  {/if}
</main>

<style>
  div.text-input-section {
    display: flex;
  }

  div.btn-container {
    text-align: center;
  }
</style>
