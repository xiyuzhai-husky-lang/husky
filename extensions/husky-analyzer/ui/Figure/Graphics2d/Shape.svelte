<script lang="ts">
    import type { Color, ShapeProps } from "";
    import Arrow from "./Shape/Arrow.svelte";

    export let shape: ShapeProps;
    export let lineWidth: number;
</script>

{#if shape.kind === "Arrow"}
    <Arrow arrow={shape} {lineWidth} />
{/if}
