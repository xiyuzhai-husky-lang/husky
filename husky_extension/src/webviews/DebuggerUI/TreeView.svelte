<script lang="ts">
    import Tree from "./TreeView/Tree.svelte";
    import { rootTraces } from "./globalState";
</script>

<div class="TreeView">
    {#each $rootTraces as trace}
        <Tree {trace} />
    {/each}
</div>

<style>
    .TreeView {
        height: 100%;
        width: grow;
        background: rgb(36, 36, 36);
    }
</style>
