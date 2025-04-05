<script lang="ts">
    import * as xdiffer from "libxdiffer";
    import {STATE_EDIT} from './shared.svelte';

    let { name, value = $bindable(), stateMachine, range = undefined } = $props();
    const uid = $props.id();
    let splittedText = $derived(xdiffer.split_by_range(value, range));
    let highlightElement: HTMLElement | undefined = $state(undefined);
    $effect(() => {
        if (splittedText) {
            highlightElement?.scrollIntoView({
                behavior: "instant",
                block: "center",
                inline: "center",
            });
        }
    });

    function onFileDrop(e: DragEvent) {
        e.preventDefault();
        const file = e.dataTransfer?.files.item(0);
        if (file) {
            const reader = new FileReader();
            reader.readAsText(file, "utf-8");
            reader.onloadend = () => {
                value = reader.result;
            };
        }
    }
</script>

<div class="pane-container">
    <div><label for="{uid}-xml">{name}</label></div>
    {#if stateMachine===STATE_EDIT}
        <textarea class="xml-text" {name} id="{uid}-xml" bind:value ondragover={(e) => e.preventDefault()} ondrop={onFileDrop}></textarea>
        <div><label for="{uid}-input-file" class="upload-label">Drop file above or click here to upload</label><input type="file" name="{uid}-input-file" id="{uid}-input-file" hidden onchange={(e) => {
            const file = e.currentTarget.files?.item(0);
            if (file) {
                const reader = new FileReader();
                reader.readAsText(file, "utf-8");
                reader.onloadend = () => {
                    value = reader.result;
                };
            }
        }}></div>
    {:else}
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

    .pane-container {
        width: 30vw;
        height: 100%;
    }

    .xml-text {
        height: 100%;
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
        white-space: nowrap;
    }

    .upload-label {
        cursor: pointer;
        background-color: #f0f0f0;
        border: 1px solid #ccc;
        padding: 8px 12px;
        border-radius: 4px;
        display: inline-block;
        font-size: 14px;
        color: #333;
    }
</style>
