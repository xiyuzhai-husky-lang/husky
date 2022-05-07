<script lang="ts">
    import type { MutationsFigureProps } from "src/trace/figure/FigureProps";
    export let figure: MutationsFigureProps;
    export let figure_control_height: number;
    export let figure_control_width: number;
</script>

{#each figure.mutations as mutation}
    <button>{mutation.name}</button>
{/each}
