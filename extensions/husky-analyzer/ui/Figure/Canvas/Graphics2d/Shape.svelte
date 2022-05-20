<script lang="ts">
    import type { Shape2dProps } from "src/figure/Graphics2d";
    import Arrow from "./Shape/Arrow.svelte";

    export let shape: Shape2dProps;
    export let lineWidth: number;
</script>

{#if shape.kind === "Arrow"}
    <Arrow arrow={shape} {lineWidth} />
{/if}
