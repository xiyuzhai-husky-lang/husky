import type { Readable, Writable } from "svelte/store";
import { writable, get } from "svelte/store";
import {
    request_trace,
    request_subtraces,
    request_activate,
    request_trace_stalk,
} from "src/websocket/websocket_client";
import type Trace from "src/trace/Trace";
import global_state from "./global_state";

export function get_root_traces_store() {
    return global_state.trace_state.root_traces_store;
}

export function get_expansion_store(trace_id: number) {
    return global_state.user_state.expansion_stores.get_store_or(
        trace_id,
        false
    );
}

export function get_shown_store(trace: Trace) {
    return global_state.user_state.shown_stores.get_store_or_with(
        trace.id,
        () => tell_shown_default(trace)
    );

    function tell_shown_default(trace: Trace) {
        switch (trace.kind) {
            case "Main":
            case "FeatureStmt":
            case "DeclStmt":
            case "FeatureBranch":
                return true;
            case "FeatureExpr":
                return false;
        }
    }
}

export function get_trace_store(trace_id: number): Readable<Trace | null> {
    return global_state.trace_state.trace_futures.get_store(trace_id, () =>
        request_trace(trace_id)
    );
}

export function get_subtraces_store(
    trace_id: number,
    effective_input_id: number | null
): Readable<Trace[] | null> {
    return global_state.trace_state.subtraces_map.get_store(
        trace_id,
        effective_input_id,
        () => request_subtraces(trace_id, effective_input_id)
    );
}

export function activate(trace_id: number) {
    request_activate(
        trace_id,
        global_state.figure_state.is_figure_cached(trace_id)
    );
}

export function get_active_trace_store(): Readable<Trace | null> {
    return global_state.user_state.active_trace_store;
}

export function get_input_id_store() {
    return global_state.user_state.input_id_store;
}

export function get_input_locked_store(): Writable<boolean> {
    return global_state.user_state.input_locked_store;
}

export function get_trace_stalk_store(trace_id: number, opt_input_id: number) {
    return global_state.trace_state.get_trace_stalk_store(
        trace_id,
        opt_input_id,
        () => {
            request_trace_stalk(trace_id, opt_input_id);
        }
    );
}

export function get_active_figure_store() {
    return global_state.figure_state.current_figure;
}

export function get_subtraces(trace_id: number) {
    console.log("todo");
}

export function get_id_before(trace_id: number) {
    throw new Error("todo");
}

export function get_id_after(trace_id: number) {
    throw new Error("todo");
}

export function toggle_expansion(trace_id: number) {
    throw new Error("todo");
}

export function is_expanded(trace_id: number): boolean {
    throw new Error("todo");
}

export function moveUp() {
    let active_trace = get(get_active_trace_store());
    if (active_trace !== null) {
        const before = get_id_before(active_trace.id);
        if (before !== undefined) {
            activate(before);
        }
    }
}

export function move_down() {
    let active_trace = get(get_active_trace_store());
    if (active_trace !== null) {
        const after = get_id_after(active_trace.id);
        if (after !== undefined) {
            return activate(after);
        }
    }
}

export function moveRight() {
    let active_trace = get(get_active_trace_store());
    if (active_trace !== null) {
        if (!is_expanded(active_trace.id) && active_trace.has_subtraces) {
            toggle_expansion(active_trace.id);
            move_down();
        }
    }
}

export function move_left() {
    let active_trace = get(get_active_trace_store());
    if (active_trace !== null) {
        if (active_trace.parent !== null) {
            toggle_expansion(active_trace.parent);
            activate(active_trace.parent);
        }
    }
}
