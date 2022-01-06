<script lang="ts">
    import type { Color, ShapeProps } from "server/types";
    import Arrow from "./Shape/Arrow.svelte";

    export let shape: ShapeProps;
    export let lineWidth: number;
</script>

{#if shape.shape_kind === "Arrow"}
    <Arrow arrow={shape} {lineWidth} />
{/if}
