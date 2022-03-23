import { derived, readable, type Readable, type Writable } from "svelte/store";
import { writable, get } from "svelte/store";
import {
    request_trace,
    request_subtraces,
    request_activate,
    request_trace_stalk,
    request_toggle_expansion,
    request_toggle_show,
} from "src/websocket/websocket_client";
import type Trace from "src/trace/Trace";
import global_state from "./global_state";

export function get_root_traces_store() {
    return global_state.trace_cache.root_traces_store;
}

export function get_expansion_store(trace_id: number) {
    return global_state.user_state.expansion_stores.get_store_or_insert(
        trace_id,
        false
    );
}

export function get_show_store(trace: Trace) {
    return global_state.user_state.shown_stores.get_store_or_insert_with(
        trace.id,
        () => {
            let result = tell_shown_default(trace);
            return result;
        }
    );

    function tell_shown_default(trace: Trace): boolean {
        switch (trace.kind) {
            case "Main":
            case "LazyStmt":
            case "StrictDeclStmt":
            case "ImprStmt":
            case "LazyBranch":
            case "CallHead":
            case "LoopFrame":
                return true;
            case "LazyExpr":
            case "StrictExpr":
                return false;
        }
    }
}

export function get_trace_future(trace_id: number): Readable<Trace | null> {
    return global_state.trace_cache.trace_futures.get_store(trace_id, () =>
        request_trace(trace_id)
    );
}

export function get_subtraces_store(
    trace_id: number,
    opt_input_id: number | null
): Readable<Trace[] | null> | null {
    let trace = global_state.trace_cache.get_trace(trace_id);
    if (trace === null) {
        return null;
    }
    let effective_input_id_for_subtraces =
        global_state.get_effective_input_id_for_subtraces(trace, opt_input_id);
    return global_state.trace_cache.subtraces_map.get_store(
        trace_id,
        effective_input_id_for_subtraces,
        () => request_subtraces(trace_id, effective_input_id_for_subtraces)
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
    return global_state.user_state.opt_input_id_store;
}

export function get_input_locked_store(): Writable<boolean> {
    return global_state.user_state.input_locked_store;
}

export function get_trace_stalk_store(trace_id: number, opt_input_id: number) {
    return global_state.trace_cache.get_trace_stalk_store(
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

export function get_subtraces(trace_id: number): Trace[] | null {
    let trace = global_state.trace_cache.get_trace(trace_id);
    if (trace === null) {
        return null;
    }
    return global_state.trace_cache.get_subtraces(
        trace_id,
        global_state.get_effective_input_id_for_subtraces(
            trace,
            get(global_state.user_state.opt_input_id_store)
        )
    );
}

export function get_id_before(trace_id: number) {
    throw new Error("todo");
}

export function get_id_after(trace_id: number) {
    throw new Error("todo");
}

export function toggle_expansion(trace_id: number) {
    request_toggle_expansion(trace_id);
}

export function toggle_show(trace_id: number) {
    request_toggle_show(trace_id);
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

export function tell_has_subtraces_store(
    trace: Trace | null
): Readable<boolean> {
    if (trace === null) {
        return readable(false);
    }
    switch (trace.kind) {
        case "Main":
        case "LazyBranch":
        case "LoopFrame":
            return readable(true);
        case "CallHead":
        case "LazyStmt":
        case "StrictDeclStmt":
            return readable(false);
        case "ImprStmt":
            return readable(trace.has_subtraces);
        case "LazyExpr":
        case "StrictExpr":
            let opt_input_id_store = global_state.user_state.opt_input_id_store;
            return derived(
                opt_input_id_store,
                ($opt_input_id_store) =>
                    $opt_input_id_store !== null && trace.has_subtraces
            );
    }
}
