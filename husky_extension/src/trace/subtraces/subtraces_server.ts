import { cache_traces } from "src/trace/trace_server";
import {
    subtraces_stores,
    subtraces_locked_on_stores_table,
} from "./subtraces_stores";
import { get } from "svelte/store";
import type Trace from "../Trace";

export function get_subtraces(id: number): Trace[] | null {
    if (id in subtraces_stores) {
        return get(subtraces_stores[id]);
    } else {
        return null;
    }
}

export function set_subtraces(
    trace_id: number,
    input_id: number | null,
    subtraces: Trace[]
) {
    cache_traces(subtraces);
    if (input_id === null) {
        if (!(trace_id in subtraces_stores)) {
            console.error("what?", trace_id, subtraces_stores);
            throw new Error("what?");
        }
        subtraces_stores[trace_id].set(subtraces);
    } else {
        if (!(trace_id in subtraces_locked_on_stores_table)) {
            throw new Error("what?");
        }
        subtraces_locked_on_stores_table[trace_id][input_id].set(subtraces);
    }
}
