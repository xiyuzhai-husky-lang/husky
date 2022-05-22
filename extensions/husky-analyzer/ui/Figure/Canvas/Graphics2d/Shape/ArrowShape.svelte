<script lang="ts">
    import type { Arrow2dShapeProps } from "src/figure/graphics2d/shape";
    import {
        get_displacement,
        get_point_distance,
        get_angle_in_degree,
    } from "src/geom2d/geom2d";

    export let arrow: Arrow2dShapeProps;
    export let lineWidth: number;

    $: displacement = get_displacement(arrow.from, arrow.to);
    $: length = get_point_distance(arrow.from, arrow.to);
    $: angle = get_angle_in_degree(displacement);
    $: arrowHeadHeight = Math.min(length / 2, gamma * lineWidth);
    const gamma = 1.7;
</script>

<g
    transform="translate({arrow.from.x} {arrow.from.y})
               rotate({angle})"
>
    <polygon
        points="
        0,{-lineWidth / 2}
        {length - arrowHeadHeight},{-lineWidth / 2}
        {length - arrowHeadHeight},{-gamma * lineWidth}
        {length},0
        {length - arrowHeadHeight},{+gamma * lineWidth}
        {length - arrowHeadHeight},{+lineWidth / 2}
        0,{+lineWidth / 2}
        "
    />
</g>
