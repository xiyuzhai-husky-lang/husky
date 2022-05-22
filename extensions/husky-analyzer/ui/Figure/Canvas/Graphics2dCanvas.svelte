<script lang="ts">
    import type { Graphics2dProps } from "src/figure/graphics2d";
    import Shape from "./Graphics2d/Shape.svelte";
    import Image from "./Graphics2d/Image.svelte";
    export let figure: Graphics2dProps;
    export let figure_canvas_height: number;
    export let figure_canvas_width: number;

    $: svgXMin = figure.xrange[0];
    $: svgWidth = figure.xrange[1] - figure.xrange[0];
    $: svgYMin = 0;
    $: svgHeight = figure.yrange[1] - figure.yrange[0];
</script>

<div class="wrapper">
    <Image
        image_layers={figure.image_layers}
        image_height={figure_canvas_height}
        image_width={figure_canvas_width}
    />

    <svg
        style="width: {figure_canvas_width}px; height: {figure_canvas_height}px"
        viewBox="{svgXMin} {svgYMin} {svgWidth} {svgHeight}"
    >
        <g transform="matrix(1 0 0 -1 0 {figure.yrange[1]})">
            {#each figure.shapes as shape2d_props}
                <Shape {shape2d_props} />
            {/each}
        </g>
    </svg>
</div>

<style>
    .wrapper {
        position: relative;
    }
    svg {
        position: absolute;
        left: 0px;
        top: 0px;
        z-index: 2;
    }
    svg :global(.red) {
        fill: red;
    }
    svg :global(.yellow) {
        fill: yellow;
    }
    svg :global(.green) {
        fill: green;
    }
    svg :global(.blue) {
        fill: blue;
    }
</style>
