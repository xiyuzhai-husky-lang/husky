<script lang="ts">
    import { focus_store, get_trace_stalk_store } from "src/data/ui";
    import Line from "./Line.svelte";
    import type Trace from "src/trace/Trace";

    export let trace: Trace;
    export let on_click: () => void;
    export let on_double_click: () => void;
    export let on_group_start_click: () => void;
    export let has_subtraces: boolean;
    export let expanded: boolean;
    export let active: boolean;

    function tell_has_extra(trace: Trace, input_id: number | null): boolean {
        switch (trace.kind) {
            case "Main":
            case "FeatureStmt":
            case "FeatureBranch":
            case "FeatureExpr":
                return input_id !== null;
            case "StrictDeclStmt":
            case "LoopFrame":
            case "ImprStmt":
            case "StrictExpr":
            case "CallHead":
                return false;
        }
    }

    $: focus = $focus_store;
    $: has_extra = tell_has_extra(trace, focus.opt_input_id);
    $: stalk_store = has_extra
        ? get_trace_stalk_store(trace.id, focus.opt_input_id!)
        : null;
    $: stalk = stalk_store !== null ? $stalk_store : null;
    $: extra_tokens = stalk !== null ? stalk.extra_tokens : [];
</script>

<div
    class="TraceNode disable-select"
    on:mousedown={on_click}
    on:dblclick={on_double_click}
    lang="ts"
>
    <div class="inner" class:active>
        {#each trace.lines as line}
            <Line
                {on_group_start_click}
                {has_subtraces}
                {expanded}
                {active}
                {line}
                {extra_tokens}
            />
        {/each}
    </div>
</div>

<style>
    .TraceNode {
        height: 24px;
        padding: 2px;
        padding-right: 3px;
    }
    .inner {
        padding: 0px;
        height: 100%;
    }
    .active {
        outline: rgb(202, 132, 2) solid 2px;
    }
</style>
