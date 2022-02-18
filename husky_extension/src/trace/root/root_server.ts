import { cache_traces } from "src/trace/trace_server";
import type Trace from "trace/Trace";
import { root_traces } from "./root_store";
import { get } from "svelte/store";

export function get_root_traces(): Trace[] | null {
    return get(root_traces);
}

export function set_root_traces(traces: Trace[]) {
    root_traces.set(traces);
    cache_traces(traces);
}
