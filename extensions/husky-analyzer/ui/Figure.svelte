<script lang="ts">
    import FlexDirection from "src/abstraction/FlexDiretion";
    import type { FigureControlData } from "src/figure/control";
    import type FigureContentProps from "src/figure";
    import FigureContent from "./Figure/Canvas.svelte";
    import FigureControl from "./Figure/Control.svelte";

    export let figure: FigureContentProps | null;
    export let figure_control_props: FigureControlData | null;
    let window_height: number;
    let figure_height: number;
    let figure_width: number;

    // flex
    $: vertical = figure_height > figure_width;
    $: figure_flex_direction = new FlexDirection(!vertical);
    $: figure_control_flex_direction = figure_flex_direction.transpose();
    // canvas
    $: figure_canvas_height = calc_figure_canvas_dimension(
        figure_height,
        figure_width
    );
    $: figure_canvas_width = figure_canvas_height;
    // control
    $: figure_control_height = calc_figure_control_height(
        vertical,
        figure_height,
        figure_canvas_height
    );
    $: figure_control_width = calc_figure_control_width(
        vertical,
        figure_width,
        figure_canvas_width
    );

    function calc_figure_canvas_dimension(
        figure_height: number,
        figure_width: number
    ): number {
        return Math.floor(0.85 * Math.min(figure_height, figure_width));
    }

    function calc_figure_control_height(
        vertical: boolean,
        figure_height: number,
        figure_canvas_height: number
    ): number {
        if (vertical) {
            return Math.floor(figure_height * 0.85) - figure_canvas_height;
        } else {
            return figure_canvas_height;
        }
    }

    function calc_figure_control_width(
        vertical: boolean,
        figure_width: number,
        figure_canvas_width: number
    ): number {
        if (vertical) {
            return figure_canvas_width;
        } else {
            return Math.floor(figure_width * 0.85) - figure_canvas_width;
        }
    }
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
        style="flex-direction: {figure_flex_direction.code()}"
    >
        <FigureContent
            {figure}
            {figure_control_props}
            {figure_canvas_height}
            {figure_canvas_width}
        />
        <FigureControl
            {figure}
            {figure_control_props}
            {figure_control_height}
            {figure_control_width}
            {figure_control_flex_direction}
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
