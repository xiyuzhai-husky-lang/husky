<script lang="ts">
    import type { Graphics2dProps } from "trace/figure";
    import Shape from "./Graphics2d/Shape.svelte";
    import Image from "./Graphics2d/Image.svelte";
    export let figure: Graphics2dProps;

    $: svgXMin = figure.xrange[0];
    $: svgWidth = figure.xrange[1] - figure.xrange[0];
    $: svgYMin = 0;
    $: svgHeight = figure.yrange[1] - figure.yrange[0];
</script>

<div class="wrapper">
    {#if figure.image !== null}
        <Image image={figure.image} />
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
        min-height: 100%;
        min-width: 100%;
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
