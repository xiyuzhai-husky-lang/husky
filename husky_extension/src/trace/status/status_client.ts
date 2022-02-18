import { shown_stores, expansion_stores } from "./status_stores";
import type { Readable, Writable } from "svelte/store";
import { writable, get } from "svelte/store";
import { shown_default, expanded_default } from "./status_default";
import { request_toggle_expansion } from "websocket/websocket_client";
import { get_trace } from "trace/trace_client";

export function is_shown(id: number): boolean {
    if (id in shown_stores) {
        return get(shown_stores[id]);
    } else {
        return get((shown_stores[id] = writable(shown_default(get_trace(id)))));
    }
}

export function get_shown_store(id: number): Writable<boolean> {
    if (id in shown_stores) {
        return shown_stores[id];
    } else {
        return (shown_stores[id] = writable(shown_default(get_trace(id))));
    }
}

export function is_expanded(id: number): boolean {
    if (id in expansion_stores) {
        return get(expansion_stores[id]);
    } else {
        return get((expansion_stores[id] = writable(expanded_default(id))));
    }
}

export function get_expansion_store(id: number): Readable<boolean> {
    if (id in expansion_stores) {
        return expansion_stores[id];
    } else {
        return (expansion_stores[id] = writable(expanded_default(id)));
    }
}

export function toggle_expansion(id: number) {
    request_toggle_expansion(id);
}
