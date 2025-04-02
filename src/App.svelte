<script lang="ts">
  import XmlTextPane from "./lib/XmlTextPane.svelte";
  import DiffTree from "./lib/DiffTree.svelte";
  import * as xdiffer from 'libxdiffer';

  let xml1 = $state("");
  let xml2 = $state("");
  let diffTree: xdiffer.DiffTree | undefined = $state(undefined);

  function onCompareBtnClick() {
    try {
      diffTree = xdiffer.build_diff_tree(xml1, xml2);
    } catch (e) {
      console.error(e);
    }
  }
</script>

<main>
  <div class="text-input-section">
    <XmlTextPane name="XML 1" bind:value={xml1}/>
    <XmlTextPane name="XML 2" bind:value={xml2}/>
  </div>
  <div class="btn-container"><button onclick={onCompareBtnClick}>Compare</button></div>
  <div><DiffTree tree={diffTree}/></div>
</main>

<style>
  div.text-input-section {
    display: flex;
  }

  div.btn-container {
    text-align: center;
  }
</style>