<script lang="ts">
    import type { Edge2d } from "src/geom2d/geom2d";
    import {
        get_displacement,
        get_point_distance,
        get_angle_in_degree,
    } from "src/geom2d/geom2d";

    export let edge: Edge2d;
    export let lineWidth: number;

    $: displacement = get_displacement(edge.from, edge.to);
    $: length = get_point_distance(edge.from, edge.to);
    $: angle = get_angle_in_degree(displacement);
    $: arrowHeadHeight = Math.min(length / 2, gamma * lineWidth);
    const gamma = 1.7;
</script>

<g
    transform="translate({edge.from.x} {edge.from.y})
               rotate({angle})"
>
    <polygon
        points="
        0,{-lineWidth / 2}
        {(length - arrowHeadHeight) / 2},{-lineWidth / 2}
        {(length - arrowHeadHeight) / 2},{-gamma * lineWidth}
        {(length + arrowHeadHeight) / 2},{-lineWidth / 2}
        {length},{-lineWidth / 2}
        {length},{+lineWidth / 2}
        {(length + arrowHeadHeight) / 2},{+lineWidth / 2}
        {(length - arrowHeadHeight) / 2},{+gamma * lineWidth}
        {(length - arrowHeadHeight) / 2},{+lineWidth / 2}
        0,{+lineWidth / 2}
        "
    />
</g>
