<script lang="ts">
    import type { Shape2dProps } from "src/figure/graphics2d/shape";
    import Arrow from "./Shape/Arrow.svelte";

    export let shape: Shape2dProps;
    export let lineWidth: number;
</script>

{#if shape.kind === "Arrow2d"}
    <Arrow arrow={shape} {lineWidth} />
{/if}
