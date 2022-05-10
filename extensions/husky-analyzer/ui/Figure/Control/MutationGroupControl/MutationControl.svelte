<script lang="ts">
    import type {
        MutationsFigureProps,
        MutationFigureProps,
    } from "src/trace/figure/FigureProps";
    export let mutation: MutationFigureProps;
    export let mutation_control_height: number;
    export let mutation_control_width: number;
    export let opt_mutation_selection: number | null;
    $: selected = opt_mutation_selection === mutation.idx;

    function select_me() {
        console.log("select me");
    }
</script>

<div
    class="MutationControl"
    style="height: {mutation_control_height}px; width: {mutation_control_width}px"
>
    <div class="MutationControlInner" class:selected>
        <div class="AssociationBox" />
        <div
            class="Name"
            style="width: {mutation_control_width - 31}px"
            on:mousedown={select_me}
        >
            {mutation.name}
        </div>
    </div>
</div>

<style>
    .MutationControl {
        display: flex;
        flex-direction: row;
        padding: 2px;
    }
    .MutationControlInner {
        display: flex;
        flex-direction: row;
    }
    .AssociationBox {
        height: 25px;
        width: 25px;
        background: rgb(47, 47, 47);
    }
    .Name {
        background: black;
        padding-left: 4px;
    }
    .selected {
        outline: orange solid 2px;
    }
</style>
