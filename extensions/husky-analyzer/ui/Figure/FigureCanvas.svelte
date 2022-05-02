<script lang="ts">
    import type FigureProps from "src/trace/figure/FigureProps";
    import Plot2d from "./Plot2d.svelte";
    import Graphics2d from "./Graphics2d.svelte";
    import PrimitiveValueFigure from "./PrimitiveValueFigure.svelte";

    export let figure: FigureProps | null;
    export let canvasHeight: number;
    export let canvasWidth: number;
</script>

{#if figure !== null && figure.kind === "Mutations"}
    <svelte:self
        figure={figure.mutations[0].after}
        {canvasHeight}
        {canvasWidth}
    />
{:else}
    <div
        class="FigureCanvas"
        style="width: {canvasWidth}px; height: {canvasHeight}px"
    >
        {#if figure !== null}
            {#if figure.kind === "Primitive"}
                <PrimitiveValueFigure {figure} />
            {:else if figure.kind === "Plot2d"}
                <Plot2d {figure} />
            {:else if figure.kind === "Graphics2d"}
                <Graphics2d {figure} />
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
