<script lang="ts">
    import Node from "./Node.svelte";
    import type GlobalState from "../GlobalState";

    export let globalState: GlobalState;
    export let activate: (idx: number) => void;
    export let toggleExpansion: (idx: number) => void;
    export let idx: number;

    $: expanded = globalState.isExpanded(idx);
    $: children = globalState.getChildren(idx);
    $: hasChildren = children && children.length > 0;
    $: trace = globalState.getTrace(idx);
</script>

<div class="TraceTree">
    <Node
        onClick={() => {
            activate(idx);
        }}
        onDoubleClick={() => {
            expanded = !expanded;
            toggleExpansion(idx);
        }}
        {trace}
        {hasChildren}
        {expanded}
        activeTrace={globalState.activeTraceIdx === idx}
    />
    {#if hasChildren && expanded === true}
        {#each children as child}
            <svelte:self
                {globalState}
                {activate}
                {toggleExpansion}
                idx={child}
            />
        {/each}
    {/if}
</div>

<style>
    .TraceTree {
        display: flex;
        flex-direction: column;
    }
</style>
