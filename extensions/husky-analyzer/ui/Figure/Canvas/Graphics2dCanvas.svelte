<script lang="ts">
    import type Graphics2dProps from "src/trace/figure/Graphics2d";
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
    {#if figure.image !== null}
        <Image
            image_props={figure.image}
            image_height={figure_canvas_height}
            image_width={figure_canvas_width}
        />
    {/if}

    <svg viewBox="{svgXMin} {svgYMin} {svgWidth} {svgHeight}">
        <g transform="matrix(1 0 0 -1 0 {figure.yrange[1]})">
            {#each figure.shape_groups as shape_group}
                <g class={shape_group.color}>
                    {#each shape_group.shapes as shape}
                        <Shape {shape} lineWidth={shape_group.line_width} />
                    {/each}
                </g>
            {/each}
        </g>
    </svg>
</div>

<style>
    .wrapper {
        position: relative;
    }
    svg {
        height: 900px;
        width: 900px;
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
