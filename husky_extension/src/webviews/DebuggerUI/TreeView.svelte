<script lang="ts">
    import Tree from "./TreeView/Tree.svelte";
    import type GlobalState from "./GlobalState";
    export let globalState: GlobalState;
    export let activate: (idx: number) => void;
    export let toggleExpansion: (idx: number) => void;
</script>

<div class="TreeView">
    <Tree {globalState} idx={0} {toggleExpansion} {activate} />
</div>

<style>
    .TreeView {
        height: 100%;
        width: grow;
        background: rgb(36, 36, 36);
    }
</style>
