<script lang="ts">
    import type FigureProps from "trace/figure/FigureProps";
    import FigureCanvas from "./Figure/Canvas.svelte";
    import FigureControl from "./Figure/Control.svelte";
    export let figure: FigureProps | null;
    let window_height: number;
    let figure_height: number;
    let figure_width: number;
    $: canvas_height = 0.9 * Math.min(figure_height, figure_width);
    $: canvas_width = canvas_height;
    $: figureContentFlexDirection =
        figure_height > figure_width ? "column" : "row";
    $: figure_control_height_percentage =
        figure_height > figure_width ? 15 : 95;
    $: figure_control_width_percentage = figure_height > figure_width ? 95 : 15;
</script>

<svelte:window bind:innerHeight={window_height} />

<div
    class="Figure disable-select"
    bind:clientHeight={figure_height}
    bind:clientWidth={figure_width}
>
    <p>title</p>
    <div
        class="FigureContent"
        style="flex-direction: {figureContentFlexDirection}"
    >
        <FigureCanvas {figure} {canvas_height} {canvas_width} />
        <FigureControl
            {figure_control_height_percentage}
            {figure_control_width_percentage}
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
</style>
