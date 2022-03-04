<script lang="ts">
    import TreeView from "./TreeView/TreeView.svelte";
    import Figure from "./Figure/Figure.svelte";
    import HSplitPane from "./SplitPane/HSplitPane.svelte";
    import { request_lock_input } from "websocket/websocket_client";
    import {
        get_active_figure_store,
        get_active_trace_store,
        get_input_id_store,
        get_input_locked_store,
        get_subtraces,
        moveRight,
        moveUp,
        move_down,
        move_left,
    } from "src/state/client";
    import { onDestroy } from "svelte";

    let window_height: number;
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
    }

    let input_locked_store = get_input_locked_store();
    let input_id_store = get_input_id_store();
    $: input = $input_id_store;
    let input_str: string = JSON.stringify(input);
    $: is_input_locked = $input_locked_store;
    const unsubscribe = input_id_store.subscribe((value) => {
        input_str = value === null ? "" : JSON.stringify(value);
    });

    onDestroy(unsubscribe);
    function on_input_clicked() {
        if (is_input_locked) {
            input_locked_store.set(false);
        }
    }
    function on_input_keydown(e: KeyboardEvent) {
        if (is_input_locked) {
            e.preventDefault();
        }
        if (e.code === "Enter" && !is_input_locked) {
            request_lock_input(input_str);
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
                    class:is_input_locked
                    on:keydown={on_input_keydown}
                    bind:value={input_str}
                    on:input={() => {}}
                />
            </div>
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
    input.is_input_locked {
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
