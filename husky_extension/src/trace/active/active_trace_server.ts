import { active_trace_store } from "./active_trace_store";
import { get } from "svelte/store";
import type Trace from "../Trace";
import { get_trace } from "../trace_server";

export function get_active_trace(): Trace | null {
    return get(active_trace_store);
}

export function did_activate(id: number) {
    let trace = get_trace(id);
    if (trace === null) {
        throw new Error("what");
    }
    active_trace_store.set(trace);
}
