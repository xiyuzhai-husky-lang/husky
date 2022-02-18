import { request_trace_stalk } from "src/websocket/websocket_client";
import type { Readable } from "svelte/store";
import { writable } from "svelte/store";
import { trace_stalk_stores_table } from "./stalk_stores";
import type TraceStalk from "./TraceStalk";

export function get_trace_stalk_store(
    trace_id: number,
    input_id: number
): Readable<TraceStalk | null> {
    let trace_stalk_stores;
    if (trace_id in trace_stalk_stores_table) {
        trace_stalk_stores = trace_stalk_stores_table[trace_id];
    } else {
        trace_stalk_stores = trace_stalk_stores_table[trace_id] = {};
    }
    if (input_id in trace_stalk_stores) {
        return trace_stalk_stores[input_id];
    } else {
        request_trace_stalk(trace_id, input_id);
        return (trace_stalk_stores[input_id] = writable(null));
    }
}
