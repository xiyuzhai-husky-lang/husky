<script lang="ts">
    import type FigureProps from "src/trace/figure/FigureProps";
    import Plot2dCanvas from "./Canvas/Plot2dCanvas.svelte";
    import Graphics2dCanvas from "./Canvas/Graphics2dCanvas.svelte";
    import PrimitiveValueCanvas from "./Canvas/PrimitiveValueCanvas.svelte";
    import NullCanvas from "./Canvas/NullCanvas.svelte";

    export let figure: FigureProps | null;
    export let figure_canvas_height: number;
    export let figure_canvas_width: number;

    $: figure_selected = get_figure_selected(figure);

    function get_figure_selected(
        figure: FigureProps | null
    ): FigureProps | null {
        if (figure === null) {
            return null;
        }
        if (figure.kind === "Mutations") {
            if (figure.mutations.length > 0) {
                return figure.mutations[0].after;
            } else {
                return null;
            }
        } else {
            return figure;
        }
    }
</script>

<div
    class="FigureCanvas"
    style="width: {figure_canvas_width}px; height: {figure_canvas_height}px"
>
    {#if figure_selected !== null}
        {#if figure_selected.kind === "Primitive"}
            <PrimitiveValueCanvas figure={figure_selected} />
        {:else if figure_selected.kind === "Plot2d"}
            <Plot2dCanvas figure={figure_selected} />
        {:else if figure_selected.kind === "Graphics2d"}
            <Graphics2dCanvas
                figure={figure_selected}
                {figure_canvas_width}
                {figure_canvas_height}
            />
        {:else}
            <p class="error">{figure_selected.kind} not supported in figure</p>
        {/if}
    {/if}
</div>

<style>
    .FigureCanvas {
        display: flex;
        border: rgb(41, 41, 41) solid;
        background: black;
    }
</style>
