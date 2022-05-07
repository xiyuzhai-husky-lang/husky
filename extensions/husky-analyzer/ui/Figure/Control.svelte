<script lang="ts">
    import type FigureProps from "src/trace/figure/FigureProps";
    import MutationsControl from "./Control/MutationsControl.svelte";
    export let figure: FigureProps | null;
    export let figure_control_height: number;
    export let figure_control_width: number;
</script>

<div
    class="FigureControl"
    style="width: {figure_control_width}px; height: {figure_control_height}px"
>
    {#if figure !== null}
        {#if figure.kind === "Mutations"}
            <MutationsControl
                {figure}
                {figure_control_height}
                {figure_control_width}
            />
        {/if}
    {/if}
</div>

<style>
    .FigureControl {
        border: rgb(41, 41, 41) solid;
    }
</style>
