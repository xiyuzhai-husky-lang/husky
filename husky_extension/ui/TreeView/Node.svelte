<script lang="ts">
    import { get_input_id_store } from "src/trace/input/input_client";
    import { get_trace_stalk_store } from "src/trace/stalk/stalk_client";
    import { Trace } from "src/trace/Trace";
    import Token from "./Token.svelte";

    export let trace: Trace;
    export let on_click: () => void;
    export let on_double_click: () => void;
    export let on_indent_click: () => void;
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
        }
    }

    const input_id_store = get_input_id_store();
    $: input_id = $input_id_store;
    $: has_extra = tell_has_extra(trace, input_id);
    $: stalk_store = has_extra
        ? get_trace_stalk_store(trace.id, input_id!)
        : null;
    $: stalk = stalk_store !== null ? $stalk_store : null;
    $: extra_tokens = [];
</script>

<div
    class="TraceNode disable-select"
    on:click={on_click}
    on:dblclick={on_double_click}
    lang="ts"
>
    <div class="inner" class:active>
        <p>
            <span
                class="indent"
                style="padding-left: {trace.indent * 9.5}px"
                on:click={on_indent_click}
            />
            <span
                class="GroupStart"
                class:has_subtraces
                class:expanded
                on:click={on_group_start_click}>&#x25b6</span
            >
            {#each trace.tokens as token}
                <Token {token} within_active_node={active} />
            {/each}
            {#if has_extra}
                {#each extra_tokens as token}
                    <Token {token} within_active_node={active} />
                {/each}
            {/if}
        </p>
    </div>
</div>

<style>
    .TraceNode {
        height: 20px;
        padding: 2px;
        padding-right: 3px;
    }
    .inner {
        padding: 0px;
    }
    .active {
        outline: rgb(202, 132, 2) solid 2px;
    }
    p {
        height: 100%;
        margin: 0;
        padding: 0;
        cursor: default;
        display: flex;
        justify-content: flex-start;
    }
    .GroupStart {
        /* cursor: pointer; */
        display: flex;
        transition: transform 50ms;
        font-size: 0px;
        text-align: center;
        justify-content: center;
        align-items: center;
        width: 18px;
    }
    .GroupStart.has_subtraces {
        font-size: 10px;
        text-align: center;
    }
    .GroupStart.has_subtraces.expanded {
        transform: rotate(90deg);
    }
</style>
