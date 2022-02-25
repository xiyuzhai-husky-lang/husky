<script lang="ts">
    import Node from "./Node.svelte";
    import { get_trace_store } from "src/trace/trace_client";
    import { get_subtraces_store } from "src/trace/subtraces/subtraces_client";
    import {
        get_expansion_store,
        toggle_expansion,
        get_shown_store,
    } from "src/trace/status/status_client";
    import {
        activate,
        get_active_trace_store,
    } from "src/trace/active/active_trace_client";
    import { get_input_id_store } from "src/trace/input/input_client";
    import type Trace from "src/trace/Trace";

    export let trace_id: number;

    $: trace_store = get_trace_store(trace_id);
    $: trace = $trace_store;
    $: expanded = get_expansion_store(trace_id);
    const input_id_store = get_input_id_store();
    $: input_id = $input_id_store;
    $: shown_store = get_shown_store(trace_id);
    $: shown = $shown_store;
    $: subtraces_store = shown ? get_subtraces_store(trace_id, input_id) : null;
    $: subtraces = shown ? $subtraces_store : null;
    $: active_trace_store = get_active_trace_store();
    $: active_trace = $active_trace_store;
    $: active = active_trace !== null ? active_trace.id === trace_id : false;
    $: locked = false;
    $: has_subtraces = tell_has_subtraces(trace, input_id);
    function toggle_expansion_locked() {
        if (!locked) {
            toggle_expansion(trace_id);
            locked = true;
            setTimeout(() => {
                locked = false;
            }, 300);
        }
    }
    function tell_has_subtraces(
        trace: Trace | null,
        opt_input_id: number | null
    ): boolean {
        if (trace === null) {
            return false;
        }
        switch (trace.kind) {
            case "Main":
                return true;
            case "FeatureStmt":
            case "DeclStmt":
                return false;
            case "FeatureBranch":
                return true;
            case "FeatureExpr":
                return opt_input_id !== null && trace.has_subtraces;
        }
    }
</script>

{#if shown && trace !== null}
    <div class="TraceTree">
        <Node
            on_click={() => {
                activate(trace_id);
            }}
            on_double_click={() => {
                toggle_expansion_locked();
            }}
            on_group_start_click={() => {
                toggle_expansion_locked();
            }}
            on_indent_click={() => {
                toggle_expansion_locked();
            }}
            {trace}
            {has_subtraces}
            expanded={$expanded}
            {active}
        />
        {#each trace.tokens as token}
            {#if token.associated_trace !== null}
                <svelte:self trace_id={token.associated_trace} />
            {/if}
        {/each}
        {#if subtraces !== null && $expanded === true}
            {#if trace.subtraces_container_class === null}
                {#each subtraces as subtrace}
                    <svelte:self trace_id={subtrace.id} />
                {/each}
            {:else if trace.subtraces_container_class === "Call"}
                <div class="Call">
                    <div class="CallTitle">call</div>
                    {#each subtraces as subtrace}
                        <svelte:self trace_id={subtrace.id} />
                    {/each}
                </div>
            {:else}
                what
            {/if}
        {/if}
    </div>
{/if}

<style>
    .TraceTree {
        display: flex;
        flex-direction: column;
    }
    .Call {
        border: white 2px solid;
        margin: 2px;
    }
    .CallTitle {
        padding-left: 10px;
        font-family: monospace;
    }
</style>
