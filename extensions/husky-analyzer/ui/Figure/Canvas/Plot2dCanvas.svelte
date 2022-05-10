<script lang="ts">
    import type { Plot2dProps } from "src/trace/figure/FigureProps";
    import Scatter2d from "./Plot2d/Scatter2d.svelte";
    export let figure: Plot2dProps;
    export let figure_canvas_height: number;
    export let figure_canvas_width: number;

    $: svgXMin = figure.xrange[0];
    $: svgWidth = figure.xrange[1] - figure.xrange[0];
    $: svgYMin = 0;
    $: svgHeight = figure.yrange[1] - figure.yrange[0];
    $: pointRadius = 0.01 * Math.min(svgWidth, svgHeight);
</script>

<svg
    style="width: {figure_canvas_width}px; height: {figure_canvas_height}px"
    viewBox="{svgXMin} {svgYMin} {svgWidth} {svgHeight}"
>
    <g><Scatter2d {figure} {pointRadius} /></g>
</svg>

<style>
    svg :global(.red) {
        fill: red;
        opacity: 0.7;
    }
    svg :global(.yellow) {
        fill: yellow;
        opacity: 0.7;
    }
    svg :global(.green) {
        fill: green;
    }
    svg :global(.blue) {
        fill: blue;
    }
</style>
