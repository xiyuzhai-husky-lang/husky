import { active_trace_id_store } from "./store";
import { writable, get } from "svelte/store";

export function get_active_trace_id(): number | null {
    return get(active_trace_id_store);
}

export function did_activate(id: number) {
    active_trace_id_store.set(id);
}
