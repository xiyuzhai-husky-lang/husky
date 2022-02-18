import { expansion_stores, shown_stores } from "./status_stores";
import { get } from "svelte/store";
import { shown_default, expanded_default } from "./status_default";
import { get_trace } from "trace/trace_server";

export function is_shown(id: number): boolean {
    if (id in shown_stores) {
        return get(shown_stores[id]);
    } else {
        return shown_default(get_trace(id));
    }
}

export function is_expanded(id: number): boolean {
    if (id in expansion_stores) {
        return get(expansion_stores[id]);
    } else {
        return expanded_default(id);
    }
}

export function did_toggle_expansion(id: number) {
    expansion_stores[id].update((expanded) => !expanded);
}

export function did_toggle_show(id: number) {
    shown_stores[id].update((shown) => !shown);
}
