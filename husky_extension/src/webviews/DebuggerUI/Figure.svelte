<script lang="ts">
    import type { FigureProps, Plot2dProps } from "trace/figure/FigureProps";
    import Plot2d from "./Figure/Plot2d.svelte";
    import Graphics2d from "./Figure/Graphics2d.svelte";
    export let figure: FigureProps;
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
        <div
            class="FigureCanvas"
            style="width: {canvasWidth}px; height: {canvasHeight}px"
        >
            {#if figure !== null}
                {#if figure.type === "Plot2d"}
                    <Plot2d {figure} />
                {:else if figure.type === "Graphics2d"}
                    <Graphics2d {figure} />
                {:else}
                    <p class="error">{figure.type} not supported</p>
                {/if}
            {/if}
        </div>
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
    .FigureCanvas {
        display: flex;
        border: rgb(41, 41, 41) solid;
        background: black;
    }
    .FigureExtra {
        border: rgb(41, 41, 41) solid;
        margin-bottom: 30px;
    }
</style>
