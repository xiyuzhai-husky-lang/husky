<script lang="ts">
    export let figure_control_height_percentage: number;
    export let figure_control_width_percentage: number;
</script>

<div
    class="FigureControl"
    style="width: {figure_control_width_percentage}%; height: {figure_control_height_percentage}%"
/>

<style>
    .FigureControl {
        border: rgb(41, 41, 41) solid;
        margin-bottom: 30px;
    }
</style>
