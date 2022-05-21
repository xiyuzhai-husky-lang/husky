<script lang="ts">
    import type {
        Shape2dKind,
        Shape2dProps,
    } from "src/figure/graphics2d/shape";
    import Arrow from "./Shape/Arrow.svelte";
    import Point2d from "./Shape/Point2d.svelte";

    export let shape2d_props: Shape2dProps;
    export let lineWidth: number;
    $: console.log(shape2d_props);
</script>

{#if shape2d_props.kind === "Arrow2d"}
    <Arrow arrow={shape2d_props} {lineWidth} />
{:else if shape2d_props.kind === "Point2d"}
    <Point2d point={shape2d_props.point} />
{/if}
