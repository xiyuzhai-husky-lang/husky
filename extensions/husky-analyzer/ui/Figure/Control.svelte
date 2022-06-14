<script lang="ts">
    import type FlexDirection from "src/abstraction/FlexDiretion";
    import type { FigureControlData } from "src/figure/control";
    import type FigureCanvasProps from "src/figure";
    import MutationsControl from "./Control/MutationGroupControl.svelte";
    export let figure: FigureCanvasProps | null;
    export let figure_control_props: FigureControlData | null;
    export let figure_control_height: number;
    export let figure_control_width: number;
    export let figure_control_flex_direction: FlexDirection;
</script>

<div
    class="FigureControl"
    style="width: {figure_control_width}px; height: {figure_control_height}px"
>
    {#if figure !== null}
        {#if figure_control_props !== null}
            {#if figure.kind === "Mutations"}
                <MutationsControl
                    {figure}
                    {figure_control_props}
                    {figure_control_height}
                    {figure_control_width}
                    {figure_control_flex_direction}
                />
            {/if}
        {:else}
            panic
        {/if}
    {/if}
</div>

<style>
    .FigureControl {
        border: rgb(41, 41, 41) solid;
        padding: 4px;
        display: flex;
    }
</style>
