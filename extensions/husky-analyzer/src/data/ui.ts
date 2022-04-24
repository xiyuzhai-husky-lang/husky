import { derived, readable, type Readable, type Writable } from "svelte/store";
import { writable, get } from "svelte/store";
import * as server from "src/server/server";
import type Trace from "src/trace/Trace";
import global from "./global";
import type FigureProps from "src/trace/figure/FigureProps";
import type Focus from "./Focus";

export const root_traces_store = global.trace_cache.root_traces_store;

export function get_expansion_store(trace_id: number) {
    return global.user_state.expansion_stores.get_store_or_insert(
        trace_id,
        false
    );
}

export const focus_store = global.user_state.focus_store;

export function get_show_store(trace: Trace) {
    return global.user_state.shown_stores.get_store_or_insert_with(
        trace.id,
        () => {
            let result = tell_shown_default(trace);
            return result;
        }
    );

    function tell_shown_default(trace: Trace): boolean {
        switch (trace.kind) {
            case "Main":
            case "FeatureStmt":
            case "StrictDeclStmt":
            case "ImprStmt":
            case "FeatureBranch":
            case "CallHead":
            case "LoopFrame":
                return true;
            case "FeatureExpr":
            case "StrictExpr":
                return false;
        }
    }
}

export function get_trace(trace_id: number): Trace {
    return global.trace_cache.traces.get(trace_id);
}

export function activate(trace_id: number) {
    const focus = get(focus_store);
    server.request_activate(
        trace_id,
        global.figure_cache.is_figure_cached(trace_id, focus) ? null : focus
    );
}

export const active_trace_store: Readable<Trace | null> =
    global.user_state.active_trace_store;

export function get_opt_input_id() {
    return get(global.user_state.focus_store);
}

export const input_locked_store: Writable<boolean> =
    global.user_state.focus_locked_store;

export function get_trace_stalk_store(trace_id: number, opt_input_id: number) {
    return global.trace_cache.get_trace_stalk_store(
        trace_id,
        opt_input_id,
        () => {
            server.request_trace_stalk(trace_id, opt_input_id);
        }
    );
}

export function get_figure(active_trace_id: number, focus: Focus): FigureProps {
    try {
        return global.figure_cache.get_figure(active_trace_id, focus);
    } catch (error) {
        console.log("focus", get(focus_store));
        throw error;
    }
}

export function get_subtraces(focus: Focus, trace_id: number): Trace[] | null {
    let trace = global.trace_cache.get_trace(trace_id);
    if (trace === null) {
        return null;
    }
    return global.trace_cache.get_subtraces(focus, trace);
}

export function get_id_before(trace_id: number) {
    throw new Error("todo");
}

export function get_id_after(trace_id: number) {
    throw new Error("todo");
}

export function toggle_expansion(trace: Trace) {
    const expanded = get(get_expansion_store(trace.id));
    const request_subtraces = !expanded
        ? !global.trace_cache.is_subtraces_cached(get(focus_store), trace)
        : false;
    server.request_toggle_expansion(
        trace.id,
        get(global.user_state.focus_store).gen_subtraces_effective_opt_input_id(
            trace
        ),
        request_subtraces
    );
}

export function toggle_show(trace_id: number) {
    server.request_toggle_show(trace_id);
}

export function is_expanded(trace_id: number): boolean {
    throw new Error("todo");
}

export function move_up() {
    let active_trace = get(active_trace_store);
    if (active_trace !== null) {
        const before = get_id_before(active_trace.id);
        if (before !== undefined) {
            activate(before);
        }
    }
}

export function move_down() {
    let active_trace = get(active_trace_store);
    if (active_trace !== null) {
        const after = get_id_after(active_trace.id);
        if (after !== undefined) {
            return activate(after);
        }
    }
}

export function moveRight() {
    let active_trace = get(active_trace_store);
    if (active_trace !== null) {
        if (!is_expanded(active_trace.id) && active_trace.has_subtraces) {
            toggle_expansion(active_trace);
            move_down();
        }
    }
}

export function move_left() {
    let active_trace = get(active_trace_store);
    if (active_trace !== null) {
        if (active_trace.parent !== null) {
            toggle_expansion(get_trace(active_trace.parent));
            activate(active_trace.parent);
        }
    }
}

export function tell_has_subtraces_store(
    trace: Trace | null
): Readable<boolean> {
    if (trace === null) {
        return readable(false);
    }
    switch (trace.kind) {
        case "Main":
        case "FeatureBranch":
        case "LoopFrame":
            return readable(true);
        case "CallHead":
        case "FeatureStmt":
        case "StrictDeclStmt":
            return readable(false);
        case "ImprStmt":
            return readable(trace.has_subtraces);
        case "FeatureExpr":
        case "StrictExpr":
            let opt_input_id_store = global.user_state.focus_store;
            return derived(
                opt_input_id_store,
                ($opt_input_id_store) =>
                    $opt_input_id_store !== null && trace.has_subtraces
            );
    }
}

export function print_state() {
    global.print_state();
}
