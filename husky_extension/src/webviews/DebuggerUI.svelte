<script lang="ts">
    import TreeView from "./DebuggerUI/TreeView.svelte";
    import Figure from "./DebuggerUI/Figure.svelte";
    import { new_globalState } from "./DebuggerUI/GlobalState";
    import type GlobalState from "./DebuggerUI/GlobalState";
    import HSplitPane from "./SplitPane/HSplitPane.svelte";

    let globalState: GlobalState = new_globalState();
    $: toggleExpansion = (idx: number) =>
        (globalState = globalState.toggleExpansion(idx));
    $: activate = (idx: number) => (globalState = globalState.activate(idx));
    $: moveUp = () => (globalState = globalState.moveUp());
    $: moveDown = () => (globalState = globalState.moveDown());
    $: moveRight = () => (globalState = globalState.moveRight());
    $: moveLeft = () => (globalState = globalState.moveLeft());
    $: onKeyDown = (e: KeyboardEvent) => {
        switch (e.code) {
            case "KeyH":
                moveLeft();
                break;
            case "KeyL":
                moveRight();
                break;
            case "KeyJ":
                moveDown();
                break;
            case "KeyK":
                moveUp();
                break;
            case "KeyS":
                console.log(JSON.stringify(globalState));
                break;
            default:
        }
    };

    let windowHeight: number;
</script>

<svelte:window on:keydown={onKeyDown} bind:innerHeight={windowHeight} />

<div class="DebuggerUI" style="height: {windowHeight}px">
    <HSplitPane>
        <TreeView slot="left" {globalState} {toggleExpansion} {activate} />
        <Figure slot="right" figure={globalState.getFigure()} />
    </HSplitPane>
</div>

<style>
    .DebuggerUI {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }
</style>
