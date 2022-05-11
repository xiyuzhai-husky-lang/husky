import { derived, readable, type Readable, type Writable } from "svelte/store";
import { writable, get } from "svelte/store";
import * as server from "src/server/server";
import type Trace from "src/trace/Trace";
import global from "./global";
import type FigureProps from "src/figure/FigureProps
import type Focus from "./Focus";
import type FigureControlProps from "src/figure/FigureControlProps

export const root_traces_store = global.trace_cache.root_traces_store;

export function get_expansion_store(trace_id: number) {
    return global.user_state.expansion_stores.get_store_or_insert(
        trace_id,
        false
    );
}

export const focus_store = global.user_state.focus_store;
 
 

export function update_figure_control_props(
    figure_control_props: FigureControlProps
) {
    let active_trace = global.user_state.active_trace();
    if (active_trace === null) {
        throw new Error("todo");
    }
    console.log("update figure control props");
    global.user_state.set_figure_control_props(
        active_trace,
        figure_control_props
    );
    let focus = get(global.user_state.focus_store);
    server.request_update_figure_control_props(
        active_trace.id,
        focus,
        figure_control_props
    );
}

export const active_trace_store: Readable<Trace | null> =
    global.user_state.active_trace_store;

export function get_opt_input_id() {
    return get(global.user_state.focus_store);
}


export function get_trace_stalk_store(trace_id: number, opt_input_id: number) {
    return global.trace_cache.get_trace_stalk_store(
        trace_id,
        opt_input_id,
        () => {
            server.request_trace_stalk(trace_id, opt_input_id);
        }
    );
} 
 
 
 
 
 

export function toggle_show(trace_id: number) {
    server.request_toggle_show(trace_id);
}

export function is_expanded(trace_id: number): boolean {
    throw new Error("todo");
}
 
