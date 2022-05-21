<script lang="ts">
    import type { Point2d } from "src/geom2d/geom2d";

    export let point: Point2d;
    $: console.log(point);
</script>

<circle cx={point.x} cy={point.y} r="0.2" />
