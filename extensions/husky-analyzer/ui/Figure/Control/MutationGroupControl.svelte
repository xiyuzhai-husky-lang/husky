<script lang="ts">
    import type FlexDirection from "src/abstraction/FlexDiretion";
    import type FigureControlProps from "src/trace/figure/FigureControlProps";

    import type { MutationsFigureProps } from "src/trace/figure/FigureProps";
    import MutationControl from "./MutationGroupControl/MutationControl.svelte";
    export let figure: MutationsFigureProps;
    export let figure_control_props: FigureControlProps;
    export let figure_control_height: number;
    export let figure_control_width: number;
    export let figure_control_flex_direction: FlexDirection;

    const mutation_control_height = 25;
    $: mutation_control_width = figure_control_width * 0.98;
</script>

<div
    class="MutationGroupControl"
    style="flex-direction: {figure_control_flex_direction}"
>
    {#each figure.mutations as mutation}
        <MutationControl
            opt_mutation_selection={figure_control_props.opt_mutation_selection}
            {mutation}
            {mutation_control_height}
            {mutation_control_width}
        />
    {/each}
</div>

<style>
    .MutationGroupControl {
        display: inline-block;
        border: 2px solid rgb(170, 50, 184);
    }
</style>
