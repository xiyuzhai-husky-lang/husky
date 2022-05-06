<script lang="ts">
    import type FigureProps from "src/trace/figure/FigureProps";
    import Plot2dCanvas from "./Canvas/Plot2dCanvas.svelte";
    import Graphics2dCanvas from "./Canvas/Graphics2dCanvas.svelte";
    import PrimitiveValueCanvas from "./Canvas/PrimitiveValueCanvas.svelte";
    import NullCanvas from "./Canvas/NullCanvas.svelte";

    export let figure: FigureProps | null;
    export let canvas_height: number;
    export let canvas_width: number;
</script>

{#if figure === null}
    <NullCanvas />
{:else if figure.kind === "Mutations"}
    {#if figure.mutations.length > 0}
        <svelte:self
            figure={figure.mutations[0].after}
            {canvas_height}
            {canvas_width}
        />
    {/if}
{:else}
    <div
        class="FigureCanvas"
        style="width: {canvas_width}px; height: {canvas_height}px"
    >
        {#if figure !== null}
            {#if figure.kind === "Primitive"}
                <PrimitiveValueCanvas {figure} />
            {:else if figure.kind === "Plot2d"}
                <Plot2dCanvas {figure} />
            {:else if figure.kind === "Graphics2d"}
                <Graphics2dCanvas {figure} />
            {:else}
                <p class="error">{figure.kind} not supported in figure</p>
            {/if}
        {/if}
    </div>
{/if}

<style>
    .FigureCanvas {
        display: flex;
        border: rgb(41, 41, 41) solid;
        background: black;
    }
</style>
