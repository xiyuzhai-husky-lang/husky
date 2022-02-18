import type { Readable } from "svelte/store";
import { get } from "svelte/store";
import { request_activate } from "src/websocket/websocket_client";
import { active_trace_store } from "./active_trace_store";
import type { Trace } from "../Trace";

export function get_active_trace(): Trace | null {
    return get(active_trace_store);
}

export function get_active_trace_store(): Readable<Trace | null> {
    return active_trace_store;
}

export function activate(trace_id: number) {
    request_activate(trace_id);
}
