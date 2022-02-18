<script lang="ts">
    import TreeView from "./TreeView/TreeView.svelte";
    import Figure from "./Figure/Figure.svelte";
    import HSplitPane from "./SplitPane/HSplitPane.svelte";
    import { get_active_figure_store } from "src/trace/figure/figure_client";
    import {
        get_active_trace_store,
        activate,
    } from "src/trace/active/active_trace_client";
    import { get_subtraces } from "src/trace/subtraces/subtraces_client";
    import {
        get_id_before,
        get_id_after,
    } from "src/trace/listing/listing_client";
    import {
        get_input_id_store,
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
    $: active_trace_store = get_active_trace_store();
    $: active_trace = $active_trace_store;
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
                if (active_trace !== null) {
                    console.log("trace: ", active_trace);
                    console.log("subtraces: ", get_subtraces(active_trace.id));
                }
                break;
            default:
        }

        function moveUp() {
            if (active_trace !== null) {
                const before = get_id_before(active_trace.id);
                if (before !== undefined) {
                    activate(before);
                }
            }
        }

        function move_down() {
            if (active_trace !== null) {
                const after = get_id_after(active_trace.id);
                if (after !== undefined) {
                    return activate(after);
                }
            }
        }

        function moveRight() {
            if (active_trace !== null) {
                if (
                    !is_expanded(active_trace.id) &&
                    active_trace.has_subtraces
                ) {
                    toggle_expansion(active_trace.id);
                    move_down();
                }
            }
        }

        function move_left() {
            if (active_trace !== null) {
                if (active_trace.parent !== null) {
                    toggle_expansion(active_trace.parent);
                    activate(active_trace.parent);
                }
            }
        }
    }

    let input_locked_store = get_input_locked_store();
    let input_id_store = get_input_id_store();
    $: input_locked = $input_locked_store;
    $: input = $input_id_store;
    function on_input_clicked() {
        if (input_locked) {
            input_locked_store.set(false);
        }
    }
    function on_input_keydown(e: KeyboardEvent) {
        if (input_locked) {
            e.preventDefault();
        }
        console.log("e.code: ", e.code, ", input_locked: ", input_locked);
        if (e.code === "Enter" && !input_locked) {
            console.log("here");
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
            locked on:
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
</style>
