<script lang="ts">
    import type {
        Shape2dKind,
        Shape2dProps,
    } from "src/figure/graphics2d/shape";
    import Arrow from "./Shape/Arrow.svelte";
    import Point2d from "./Shape/Point2d.svelte";

    export let shape2d_props: Shape2dProps;
    $: console.log(shape2d_props);
</script>

{#if shape2d_props.kind === "Arrow2d"}
    <Arrow arrow={shape2d_props} lineWidth={1.0} />
{:else if shape2d_props.kind === "Point2d"}
    <Point2d point={shape2d_props.point} />
{/if}
<!-- 
{#each figure.shapes as shape_group}
<g class={shape_group.color}>
    {#each shape_group.shapes as shape2d_props}
        <Shape
            {shape2d_props}
            lineWidth={shape_group.line_width}
        />
    {/each}
</g>
{/each} -->
