<script lang="ts">
    import TreeView from "./DebuggerUI/TreeView.svelte";
    import Figure from "./DebuggerUI/Figure.svelte";
    import HSplitPane from "./SplitPane/HSplitPane.svelte";
    import { get_active_figure_store } from "src/trace/figure/figure_client";
    import { get_active_trace_id_store, activate } from "trace/activate/client";
    import { get_trace } from "src/trace/trace_client";
    import { get_subtraces, has_children } from "src/trace/subtraces/client";
    import {
        get_id_before,
        get_id_after,
    } from "src/trace/listing/listing_client";
    import {
        get_input_store,
        get_input_locked_store,
    } from "src/trace/input/input_client";
    import {
        toggle_expansion,
        is_expanded,
    } from "src/trace/status/status_client";
    import { request_lock_input } from "websocket/websocket_client";

    let window_height: number;
    let input: number | null = null;
    let input_temp: number | null = null;
    $: figure_store = get_active_figure_store();
    $: figure = $figure_store;
    $: active_trace_id_store = get_active_trace_id_store();
    $: active_trace_id = $active_trace_id_store;
    function on_key_down(e: KeyboardEvent) {
        switch (e.code) {
            case "KeyH":
                move_left();
                break;
            case "KeyL":
                moveRight();
                break;
            case "KeyJ":
                move_down();
                break;
            case "KeyK":
                moveUp();
                break;
            case "KeyT":
                if (active_trace_id !== null) {
                    console.log("trace: ", get_trace(active_trace_id));
                    console.log("subtraces: ", get_subtraces(active_trace_id));
                }
                break;
            default:
        }

        function moveUp() {
            if (active_trace_id !== null) {
                const before = get_id_before(active_trace_id);
                if (before !== undefined) {
                    activate(before);
                }
            }
        }

        function move_down() {
            if (active_trace_id !== null) {
                const after = get_id_after(active_trace_id);
                if (after !== undefined) {
                    return activate(after);
                }
            }
        }

        function moveRight() {
            if (active_trace_id !== null) {
                if (
                    !is_expanded(active_trace_id) &&
                    has_children(active_trace_id)
                ) {
                    toggle_expansion(active_trace_id);
                    move_down();
                }
            }
        }

        function move_left() {
            if (active_trace_id !== null) {
                const trace = get_trace(active_trace_id);
                if (trace !== null) {
                    if (trace.parent !== null) {
                        toggle_expansion(trace.parent);
                        activate(trace.parent);
                    }
                }
            }
        }
    }

    let input_locked_store = get_input_locked_store();
    let input_store = get_input_store();
    $: input_locked = $input_locked_store;
    $: input = $input_store;
    function on_input_clicked() {
        if (input_locked) {
            input_locked_store.set(false);
        }
    }
    function on_input_keydown(e: KeyboardEvent) {
        if (input_locked) {
            e.preventDefault();
        }
        if (e.code === "Enter" && !input_locked) {
            request_lock_input(input_temp);
        }
    }
</script>

<svelte:window on:keydown={on_key_down} bind:innerHeight={window_height} />

<div class="DebuggerUI" style="height: {window_height}px">
    <HSplitPane>
        <TreeView slot="left" />
        <Figure slot="right" {figure} />
    </HSplitPane>
    <div class="Bottom">
        <div class="BottomItem">
            lock on:
            <div on:click={on_input_clicked} class="BottomInputWrapper">
                <input
                    class:input_locked
                    on:keydown={on_input_keydown}
                    bind:value={input_temp}
                />
            </div>
            {#if input !== null}
                =
                {input}
            {/if}
        </div>
    </div>
</div>

<style>
    .DebuggerUI {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }
    .Bottom {
        padding-left: 10px;
        display: flex;
        flex-direction: row;
        height: 25px;
        background: rgb(0, 126, 126);
        display: flex;
        align-items: center;
    }
    input {
        max-height: 16px;
        max-width: 32px;
        background: rgb(0, 105, 105);
        border: none;
        outline: orange solid 1px;
        color: white;
    }
    input.input_locked {
        caret-color: transparent;
        background: rgb(0, 80, 80);
        outline: none;
    }
    .BottomItem {
        display: flex;
        flex-direction: row;
        align-items: center;
    }
    .BottomInputWrapper {
        display: flex;
        align-items: center;
        padding-left: 10px;
        padding-right: 10px;
    }
    .Package {
        background: rgb(0, 80, 80);
        padding-left: 3px;
        padding-right: 3px;
    }
</style>
