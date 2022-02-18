import {
    subtraces_stores,
    subtraces_locked_on_stores_table,
} from "./subtraces_stores";
import type { Readable, Writable } from "svelte/store";
import { writable, get } from "svelte/store";
import { request_subtraces } from "src/websocket/websocket_client";
import type Trace from "../Trace";
import { get_trace } from "trace/trace_client";
import { get_input } from "trace/input/input_client";

function get_effective_input_id(trace_id: number): number | null {
    let trace = get_trace(trace_id);
    if (trace === null) {
        return null;
    }
    switch (trace.kind) {
        case "Main":
            return null;
        case "FeatureStmt":
            return null;
        case "FeatureBranch":
            return null;
        case "FeatureExpr":
            return get_input();
    }
}

function get_effective_input_id_or(
    trace_id: number,
    default_input_id: number | null
): number | null {
    let trace = get_trace(trace_id);
    if (trace === null) {
        return null;
    }
    switch (trace.kind) {
        case "Main":
            return null;
        case "FeatureStmt":
            return null;
        case "FeatureBranch":
            return null;
        case "FeatureExpr":
            return default_input_id;
    }
}

export function get_subtraces(trace_id: number): Trace[] | null {
    let input_id = get_effective_input_id(trace_id);
    if (input_id === null) {
        if (trace_id in subtraces_stores) {
            return get(subtraces_stores[trace_id]);
        } else {
            request_subtraces(trace_id, input_id);
            return get((subtraces_stores[trace_id] = writable(null)));
        }
    } else {
        let subtraces_locked_on_stores: {
            [trace_id: number]: Writable<Trace[] | null>;
        };
        if (trace_id in subtraces_locked_on_stores_table) {
            subtraces_locked_on_stores =
                subtraces_locked_on_stores_table[trace_id];
        } else {
            subtraces_locked_on_stores = subtraces_locked_on_stores_table[
                trace_id
            ] = {};
        }
        if (!(input_id in subtraces_locked_on_stores)) {
            request_subtraces(trace_id, input_id);
            return get((subtraces_locked_on_stores[input_id] = writable(null)));
        } else {
            return get(subtraces_locked_on_stores[input_id]);
        }
    }
}

export function has_children(id: number): boolean {
    let subtraces = get_subtraces(id);
    if (subtraces !== null) {
        return subtraces.length > 0;
    } else {
        return false;
    }
}

export function get_subtraces_store(
    trace_id: number,
    default_input_id: number | null
): Readable<Trace[] | null> {
    let input_id = get_effective_input_id_or(trace_id, default_input_id);
    if (input_id === null) {
        if (trace_id in subtraces_stores) {
            return subtraces_stores[trace_id];
        } else {
            request_subtraces(trace_id, input_id);
            return (subtraces_stores[trace_id] = writable(null));
        }
    } else {
        let subtraces_locked_on_stores: {
            [trace_id: number]: Writable<Trace[] | null>;
        };
        if (trace_id in subtraces_locked_on_stores_table) {
            subtraces_locked_on_stores =
                subtraces_locked_on_stores_table[trace_id];
        } else {
            subtraces_locked_on_stores = subtraces_locked_on_stores_table[
                trace_id
            ] = {};
        }
        if (!(input_id in subtraces_locked_on_stores)) {
            request_subtraces(trace_id, input_id);
            return (subtraces_locked_on_stores[input_id] = writable(null));
        } else {
            return subtraces_locked_on_stores[input_id];
        }
    }
}
