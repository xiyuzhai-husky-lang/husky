<script lang="ts">
    import Node from "./Node.svelte";
    import Associated from "./Associated.svelte";
    import {
        toggleExpansion,
        activate,
        activeTraceIdx,
        isExpanded,
        getSubtraces,
    } from "../globalState";
    import { Trace } from "src/server/types";

    export let trace: Trace;
    let expanded = isExpanded(trace.id);
    let subtraces = getSubtraces(trace.id);
    let locked = false;
    $: has_subtraces = $subtraces !== null && $subtraces.length > 0;
    function toggleExpansionLocked() {
        if (!locked) {
            toggleExpansion(trace.id);
            locked = true;
            setTimeout(() => {
                locked = false;
            }, 300);
        }
    }
</script>

<div class="TraceTree">
    <Node
        on_click={() => {
            activate(trace.id);
        }}
        on_double_click={() => {
            toggleExpansionLocked();
        }}
        on_group_start_click={() => {
            toggleExpansionLocked();
        }}
        on_indent_click={() => {
            toggleExpansionLocked();
        }}
        {trace}
        {has_subtraces}
        expanded={$expanded}
        active={$activeTraceIdx === trace.id}
    />
    {#each trace.tokens as token}
        {#if token.associated_trace !== null}
            <Associated associated_trace_id={token.associated_trace} />
        {/if}
    {/each}
    {#if $subtraces !== null && $expanded === true}
        {#each $subtraces as trace}
            <svelte:self {trace} />
        {/each}
    {/if}
</div>

<style>
    .TraceTree {
        display: flex;
        flex-direction: column;
    }
</style>
