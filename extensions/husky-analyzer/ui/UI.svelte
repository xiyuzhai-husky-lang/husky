<script lang="ts">
    import TreeView from "./TreeView.svelte";
    import Figure from "./Figure.svelte";
    import HSplitPane from "./SplitPane/HSplitPane.svelte";
    import { request_lock_input } from "src/server/server";
    import {
        active_trace_store,
        focus_store,
        input_locked_store,
        get_subtraces,
        moveRight as move_right,
        move_up,
        move_down,
        move_left,
        get_figure,
        print_state,
        get_figure_control_store,
    } from "src/data/ui";
    import { onDestroy } from "svelte";

    let window_height: number;
    $: bottom_height = window_height * 0.03;
    $: middle_height = window_height - bottom_height;

    $: focus = $focus_store;
    $: opt_input_id = focus.opt_input_id;
    $: active_trace = $active_trace_store;
    $: figure =
        active_trace !== null ? get_figure(active_trace.id, focus) : null;
    $: figure_control_store =
        active_trace !== null ? get_figure_control_store(active_trace) : null;
    $: figure_control_props =
        active_trace !== null ? $figure_control_store : null;

    function on_key_down(e: KeyboardEvent) {
        switch (e.code) {
            case "KeyH":
                move_left();
                break;
            case "KeyL":
                move_right();
                break;
            case "KeyJ":
                move_down();
                break;
            case "KeyK":
                move_up();
                break;
            case "KeyT":
                if (active_trace !== null) {
                    console.log("trace: ", active_trace);
                    console.log(
                        "subtraces: ",
                        get_subtraces(focus, active_trace.id)
                    );
                }
                break;
            case "KeyS":
                print_state();
                break;
            case "KeyF":
                console.log("figure props: ", figure);
                console.log("figure control props: ", figure_control_props);
            default:
        }
    }

    let input_str: string = JSON.stringify(opt_input_id);
    $: is_input_locked = $input_locked_store;
    const unsubscribe = focus_store.subscribe((focus) => {
        input_str =
            focus.opt_input_id === null
                ? ""
                : JSON.stringify(focus.opt_input_id);
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
    <div class="Middle" style="height: {middle_height}px">
        <HSplitPane>
            <TreeView slot="left" />
            <Figure slot="right" {figure} {figure_control_props} />
        </HSplitPane>
    </div>
    <div class="Bottom" style="height: {bottom_height}px">
        <div class="BottomItem">
            focus:
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
        overflow: none;
    }
    .Bottom {
        padding-left: 10px;
        display: flex;
        flex-direction: row;
        height: 35px;
        background: rgb(0, 126, 126);
        display: flex;
        align-items: center;
    }
    input {
        min-width: 80%;
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
        width: 100%;
        padding-left: 5px;
    }
    .BottomInputWrapper {
        display: flex;
        align-items: center;
        padding-left: 10px;
        padding-right: 10px;
        width: 50%;
    }
</style>
