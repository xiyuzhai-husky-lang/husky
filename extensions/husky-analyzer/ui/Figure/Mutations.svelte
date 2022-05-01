<script lang="ts">
    import type MutationsFigureProps from "src/trace/figure/Mutations";
    import Visual from "./Visual/Visual.svelte";

    export let figure: MutationsFigureProps;
</script>

<Visual visual={figure.mutations[0].after} />
