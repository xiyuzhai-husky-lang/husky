import { cache_traces } from "src/trace/trace_server";
import { subtraces_stores } from "./stores";
import { get } from "svelte/store";
import type Trace from "../Trace";

export function get_subtraces(id: number): Trace[] | null {
    if (id in subtraces_stores) {
        return get(subtraces_stores[id]);
    } else {
        return null;
    }
}

export function set_subtraces(id: number, subtraces: Trace[]) {
    if (!(id in subtraces_stores)) {
        console.error(id, subtraces_stores);
    }
    subtraces_stores[id].set(subtraces);
    cache_traces(subtraces);
}
