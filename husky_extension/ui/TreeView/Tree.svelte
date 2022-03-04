<script lang="ts">
    import Node from "./Node.svelte";
    import {
        get_trace_future,
        get_expansion_store,
        get_show_store,
        get_subtraces_store,
        get_active_trace_store,
        activate,
        toggle_expansion,
    } from "src/state/client";
    import {
        tell_has_subtraces_store,
        get_input_id_store,
    } from "src/state/client";

    export let trace_id: number;

    $: trace_future = get_trace_future(trace_id);
    $: trace = $trace_future;
    $: expanded_store = get_expansion_store(trace_id);
    $: expanded = $expanded_store;
    $: shown_store = trace !== null ? get_show_store(trace) : null;
    $: shown = shown_store !== null ? $shown_store : false;
    $: opt_input_id_store = get_input_id_store();
    $: subtraces_store = shown
        ? get_subtraces_store(trace_id, $opt_input_id_store)
        : null;
    $: subtraces = shown ? $subtraces_store : null;
    $: active_trace_store = get_active_trace_store();
    $: active_trace = $active_trace_store;
    $: active = active_trace !== null ? active_trace.id === trace_id : false;
    $: locked = false;
    $: has_subtraces_store = tell_has_subtraces_store(trace);
    $: has_subtraces = $has_subtraces_store;
    function toggle_expansion_locked() {
        if (!locked) {
            toggle_expansion(trace_id);
            locked = true;
            setTimeout(() => {
                locked = false;
            }, 300);
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
            {expanded}
            {active}
        />
        {#each trace.tokens as token}
            {#if token.associated_trace !== null}
                <svelte:self trace_id={token.associated_trace} />
            {/if}
        {/each}
        {#if subtraces !== null && expanded === true && has_subtraces}
            {#if trace.subtraces_container_class === null}
                {#each subtraces as subtrace}
                    <svelte:self trace_id={subtrace.id} />
                {/each}
            {:else if trace.subtraces_container_class === "Call"}
                <div class="Call">
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
        font-size: 14px;
    }
</style>
