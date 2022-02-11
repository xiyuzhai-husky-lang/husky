<script lang="ts">
    import { TokenProps } from "src/server/types";
    import { getActiveAssociatedTrace } from "../globalState";
    import Tree from "./Tree.svelte";

    export let associated_trace_id: number;
    $: activeAssociatedTrace = getActiveAssociatedTrace(associated_trace_id);
    $: console.log($activeAssociatedTrace);
</script>

{#if $activeAssociatedTrace !== null}
    <Tree trace={$activeAssociatedTrace} />
{/if}
