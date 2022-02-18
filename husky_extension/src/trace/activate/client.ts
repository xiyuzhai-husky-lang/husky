import type { Readable } from "svelte/store";
import { get } from "svelte/store";
import { request_activate } from "src/websocket/websocket_client";
import { active_trace_id_store } from "./store";

export function get_active_trace_id(): number | null {
    return get(active_trace_id_store);
}

export function get_active_trace_id_store(): Readable<number | null> {
    return active_trace_id_store;
}

export function activate(id: number) {
    request_activate(id);
}
