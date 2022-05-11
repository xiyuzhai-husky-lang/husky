<script lang="ts">
    import type { ShapeProps } from "src/figure/src/figure/Graphics2drt Arrow from "./Shape/Arrow.svelte";

    export let shape: ShapeProps;
    export let lineWidth: number;
</script>

{#if shape.kind === "Arrow"}
    <Arrow arrow={shape} {lineWidth} />
{/if}
