<script lang="ts">
    import type { Shape2dProps } from "src/figure/graphics2d/shape";
    import Arrow from "./Shape/ArrowShape.svelte";
    import Contour from "./Shape/ContourShape.svelte";
    import Point2d from "./Shape/Point2dShape.svelte";

    export let shape2d_props: Shape2dProps;
</script>

{#if shape2d_props.kind === "Arrow2d"}
    <Arrow arrow={shape2d_props} lineWidth={1.0} />
{:else if shape2d_props.kind === "Point2d"}
    <Point2d point={shape2d_props.point} />
{:else if shape2d_props.kind === "Contour"}
    <Contour points={shape2d_props.points} />
{/if}

<style>
    text {
        font-size: 0.08em;
    }
</style>
