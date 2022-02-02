<script lang="ts">
    import Node from "./Node.svelte";
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
    $: hasSubtraces = $subtraces !== null && $subtraces.length > 0;
</script>

<div class="TraceTree">
    <Node
        onClick={() => {
            activate(trace.id);
        }}
        onDoubleClick={() => {
            toggleExpansion(trace.id);
        }}
        {trace}
        {hasSubtraces}
        expanded={$expanded}
        active={$activeTraceIdx === trace.id}
    />
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
