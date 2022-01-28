<script lang="ts">
    import TreeView from "./DebuggerUI/TreeView.svelte";
    import Figure from "./DebuggerUI/Figure.svelte";
    import { figure, onKeyDown, getDummy } from "./DebuggerUI/globalState";
    import HSplitPane from "./SplitPane/HSplitPane.svelte";

    let windowHeight: number;
</script>

<svelte:window on:keydown={onKeyDown} bind:innerHeight={windowHeight} />

<div class="DebuggerUI" style="height: {windowHeight}px">
    <HSplitPane>
        <TreeView slot="left" />
        <Figure slot="right" figure={$figure} />
    </HSplitPane>
    <div class="BottomTemp">connection: {getDummy()}</div>
</div>

<style>
    .DebuggerUI {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }
    .BottomTemp {
        display: flex;
        height: 25px;
        background: rgb(0, 126, 126);
    }
</style>
