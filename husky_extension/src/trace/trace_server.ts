import type Trace from "./Trace";
import { trace_stores } from "./trace_stores";
import { writable, get } from "svelte/store";

export function get_trace(id: number): Trace | null {
    if (!(id in trace_stores)) {
        return null;
    } else {
        return get(trace_stores[id]);
    }
}

export function cache_traces(traces: Trace[]) {
    for (const trace of traces) {
        cache_trace(trace);
    }
}

export function cache_trace(trace: Trace) {
    let id = trace.id;
    if (!(id in trace_stores)) {
        trace_stores[id] = writable(trace);
    } else {
        trace_stores[id].set(trace);
    }
}
