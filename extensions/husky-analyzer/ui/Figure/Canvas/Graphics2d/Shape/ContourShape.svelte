<script lang="ts">
    import type { Point2d, Edge2d } from "src/geom2d/geom2d";
    import Edge2dShape from "./Edge2dShape.svelte";

    export let points: Point2d[];
    export let closed: boolean = true;
    $: edges = calc_edges(points, closed);

    function calc_edges(points: Point2d[], closed: boolean): Edge2d[] {
        let edges: Edge2d[] = [];
        for (let i = 0; i < points.length - 1; i++) {
            edges.push({
                from: points[i],
                to: points[i + 1],
            });
        }
        if (closed) {
            edges.push({ from: points[points.length - 1], to: points[0] });
        }
        return edges;
    }
</script>

{#each points as point}
    <circle cx={point.x} cy={point.y} r="0.2" />
{/each}

{#each edges as edge}
    <Edge2dShape {edge} lineWidth={0.1} />
{/each}
