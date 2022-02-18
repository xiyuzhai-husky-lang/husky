<script lang="ts">
    import Tree from "./Tree.svelte";
    import { get_root_traces_store } from "src/trace/root/root_client";

    let root_traces_store = get_root_traces_store();
    $: root_traces = $root_traces_store;
</script>

<div class="TreeView">
    {#if root_traces !== null}
        {#each root_traces as trace}
            <Tree trace_id={trace.id} />
        {/each}
    {/if}
</div>

<style>
    .TreeView {
        height: 100%;
        width: grow;
        background: rgb(36, 36, 36);
    }
</style>
