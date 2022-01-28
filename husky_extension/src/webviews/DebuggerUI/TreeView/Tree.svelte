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
    $: idx = trace.id;

    let expanded = isExpanded(idx);
    let subtraces = getSubtraces(idx);
    $: hasSubtraces = $subtraces !== null && $subtraces.length > 0;
</script>

<div class="TraceTree">
    <Node
        onClick={() => {
            activate(idx);
        }}
        onDoubleClick={() => {
            toggleExpansion(idx);
        }}
        {trace}
        {hasSubtraces}
        expanded={$expanded}
        active={$activeTraceIdx === idx}
    />
    {#if $subtraces !== null && $expanded === true}
        {#each $subtraces as child}
            <svelte:self idx={child} />
        {/each}
    {/if}
</div>

<style>
    .TraceTree {
        display: flex;
        flex-direction: column;
    }
</style>
