<script lang="ts">
    import type { Plot2dProps } from "trace/figure";
    export let figure: Plot2dProps;
    export let pointRadius: number;

    $: transformToSvgY = (y: number) => {
        return figure.yrange[1] - y;
    };
</script>

{#each figure.groups as group}
    {#each group.points as point}
        <circle
            class={group.color}
            cx={point.x}
            cy={transformToSvgY(point.y)}
            r={pointRadius}
        />
    {/each}
{/each}
