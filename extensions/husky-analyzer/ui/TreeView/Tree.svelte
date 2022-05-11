<script lang="ts">
    import Node from "./Node.svelte";
    import state, { active_trace_store, focus_store } from "src/state";
    export let trace_id: number;

    $: trace = state.get_trace(trace_id);
    $: expanded_store = state.get_expansion_store(trace_id);
    $: expanded = $expanded_store;
    $: shown_store = state.get_show_store(trace);
    $: shown = $shown_store;
    $: subtraces =
        shown && expanded ? state.get_subtraces($focus_store, trace_id) : null;
    $: active_trace = $active_trace_store;
    $: active = active_trace !== null ? active_trace.id === trace_id : false;
    let locked = false;
    $: has_subtraces_store = state.gen_has_subtraces_store(trace);
    $: has_subtraces = $has_subtraces_store;
    function toggle_expansion_locked() {
        if (has_subtraces && !locked) {
            state.toggle_expansion(trace);
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
                state.activate(trace_id);
            }}
            on_double_click={() => {
                toggle_expansion_locked();
            }}
            on_group_start_click={() => {
                toggle_expansion_locked();
            }}
            {trace}
            {has_subtraces}
            {expanded}
            {active}
        />
        {#each trace.lines as line}
            {#each line.tokens as token}
                {#if token.value != "]" && token.associated_trace !== null}
                    <svelte:self trace_id={token.associated_trace} />
                {/if}
            {/each}
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
</style>
