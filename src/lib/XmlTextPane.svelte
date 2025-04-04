<script lang="ts">
    import * as xdiffer from "libxdiffer";
    import {STATE_EDIT, STATE_COMPARE} from './shared.svelte';

    let { name, value = $bindable(), stateMachine, range = undefined } = $props();
    const uid = $props.id();
    let splittedText = $derived(xdiffer.split_by_range(value, range));
    let highlightElement: HTMLElement | undefined = $state(undefined);
    $effect(() => {
        if (splittedText) {
            highlightElement?.scrollIntoView({
                behavior: "instant",
                block: "center",
                inline: "nearest",
            });
        }
    });
</script>

<div class="pane-container">
    <div><label for="{uid}-xml">{name}</label></div>
    {#if stateMachine===STATE_EDIT}
        <textarea class="xml-text" {name} id="{uid}-xml" bind:value></textarea>
    {:else if stateMachine === STATE_COMPARE}
        <div class="xml-text">
            <pre><code>{splittedText.head()}</code><code bind:this={highlightElement} class="highlight-text"
                    >{splittedText.middle()}</code
                ><code>{splittedText.tail()}</code></pre>
        </div>
    {/if}
</div>

<style>
    label {
        font-weight: bold;
    }

    .xml-text {
        width: 30vw;
        height: 80vh;
        overflow: scroll;
    }

    .highlight-text {
        background-color: #ffeb3b;
        color: black;
    }

    textarea {
        width: 90%;
        height: 100%;
        resize: none;
        font-family: monospace;
        font-size: 14px;
    }
</style>
