<script lang="ts">
    import type VisualProps from "src/trace/figure/VisualProps";

    export let visual: VisualProps;
</script>

{#if visual.kind == "BinaryImage28"}
    BinaryImage28TODO
{:else}
    Not handled case with visual.kind = {visual.kind}
{/if}
