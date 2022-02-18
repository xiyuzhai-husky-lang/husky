import { subtraces_stores } from "./stores";
import type { Readable } from "svelte/store";
import { writable, get } from "svelte/store";
import { request_subtraces } from "src/websocket/websocket_client";
import type Trace from "../Trace";

export function get_subtraces(id: number): Trace[] | null {
    if (id in subtraces_stores) {
        return get(subtraces_stores[id]);
    } else {
        request_subtraces(id);
        return get((subtraces_stores[id] = writable(null)));
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

export function get_subtraces_store(id: number): Readable<Trace[] | null> {
    if (id in subtraces_stores) {
        return subtraces_stores[id];
    } else {
        request_subtraces(id);
        return (subtraces_stores[id] = writable(null));
    }
}
