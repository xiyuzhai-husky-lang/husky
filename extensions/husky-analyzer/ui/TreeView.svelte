<script lang="ts">
    import Tree from "./TreeView/Tree.svelte";
    import { root_traces_store } from "src/state";
    $: root_traces = $root_traces_store;
</script>

<div class="TreeView">
    <div class="TreeViewContent">
        {#if root_traces !== null}
            {#each root_traces as trace}
                <Tree trace_id={trace.id} />
            {/each}
        {/if}
    </div>
</div>

<style>
    .TreeView {
        height: 100%;
        background: rgb(36, 36, 36);
        overflow-y: scroll;
        scrollbar-width: none;
    }
    .TreeView::-webkit-scrollbar {
        display: none;
    }
</style>
