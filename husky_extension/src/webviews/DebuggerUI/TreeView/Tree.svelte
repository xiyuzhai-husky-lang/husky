<script lang="ts">
    import Node from "./Node.svelte";
    import { get_trace_store } from "src/trace/trace_client";
    import { get_subtraces_store } from "src/trace/subtraces/client";
    import {
        get_expansion_store,
        toggle_expansion,
        get_shown_store,
    } from "src/trace/status/status_client";
    import {
        activate,
        get_active_trace_id_store,
    } from "src/trace/activate/client";

    export let trace_id: number;

    $: trace_store = get_trace_store(trace_id);
    $: trace = $trace_store;
    $: expanded = get_expansion_store(trace_id);
    $: subtraces_store = get_subtraces_store(trace_id);
    $: subtraces = $subtraces_store;
    $: active_trace_id_store = get_active_trace_id_store();
    $: active = $active_trace_id_store === trace_id;
    $: locked = false;
    $: shown_store = get_shown_store(trace_id);
    $: shown = $shown_store;
    $: has_subtraces = subtraces !== null && subtraces.length > 0;
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
            expanded={$expanded}
            {active}
        />
        {#each trace.tokens as token}
            {#if token.associated_trace !== null}
                <svelte:self trace_id={token.associated_trace} />
            {/if}
        {/each}
        {#if subtraces !== null && $expanded === true}
            {#each subtraces as subtrace}
                <svelte:self trace_id={subtrace.id} />
            {/each}
        {/if}
    </div>
{/if}

<style>
    .TraceTree {
        display: flex;
        flex-direction: column;
    }
</style>
