import type Trace from "./Trace";
import { trace_stores } from "./trace_stores";
import type { Readable, Writable } from "svelte/store";
import { writable, get } from "svelte/store";
import { request_trace } from "src/websocket/websocket_client";

export function get_trace(id: number): Trace | null {
    if (!(id in trace_stores)) {
        request_trace(id);
        return get((trace_stores[id] = writable(null)));
    } else {
        return get(trace_stores[id]);
    }
}

export function get_trace_store(id: number): Readable<Trace | null> {
    if (!(id in trace_stores)) {
        return (trace_stores[id] = writable(null));
    } else {
        return trace_stores[id];
    }
}
