<script lang="ts">
    import type FigureProps from "trace/figure/FigureProps";
    import FigureCanvas from "./FigureCanvas.svelte";
    export let figure: FigureProps | null;
    let windowHeight: number;
    let figureHeight: number;
    let figureWidth: number;
    $: canvasHeight = 0.9 * Math.min(figureHeight, figureWidth);
    $: canvasWidth = canvasHeight;
    $: figureContentFlexDirection =
        figureHeight > figureWidth ? "column" : "row";
    $: figureExtraHeightPercentage = figureHeight > figureWidth ? 15 : 95;
    $: figureExtraWidthPercentage = figureHeight > figureWidth ? 95 : 15;
</script>

<svelte:window bind:innerHeight={windowHeight} />

<div
    class="Figure disable-select"
    bind:clientHeight={figureHeight}
    bind:clientWidth={figureWidth}
>
    <p>title</p>
    <div
        class="FigureContent "
        style="flex-direction: {figureContentFlexDirection}"
    >
        <FigureCanvas {figure} {canvasHeight} {canvasWidth} />
        <div
            class="FigureExtra"
            style="width: {figureExtraWidthPercentage}%; height: {figureExtraHeightPercentage}%"
        />
    </div>
</div>

<style>
    .Figure {
        display: flex;
        flex-direction: column;
        justify-content: space-around;
        align-items: center;
        width: 100%;
        height: 100%;
        background: rgb(101, 163, 165);
    }
    .FigureContent {
        width: 95%;
        height: 97%;
        display: flex;
        justify-content: space-around;
        align-items: center;
    }
    .FigureExtra {
        border: rgb(41, 41, 41) solid;
        margin-bottom: 30px;
    }
</style>
