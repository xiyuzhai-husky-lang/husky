<script lang="ts">
    import type {
        MutationsFigureProps,
        MutationFigureProps,
    } from "src/trace/figure/FigureProps";
    export let mutation: MutationFigureProps;
    export let mutation_control_height: number;
    export let mutation_control_width: number;
</script>

<div
    class="MutationControl"
    style="height: {mutation_control_height}px; width: {mutation_control_width}px"
>
    <div class="AssociationBox" />
    <div class="Name" style="width: {mutation_control_width - 31}px">
        {mutation.name}
    </div>
</div>

<style>
    .MutationControl {
        display: flex;
        flex-direction: row;
        padding: 2px;
    }
    .AssociationBox {
        height: 25px;
        width: 25px;
        background: pink;
    }
    .Name {
        background: black;
        padding-left: 4px;
    }
</style>
