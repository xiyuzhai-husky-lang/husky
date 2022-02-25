import type Trace from "trace/Trace";
import type { Readable } from "svelte/store";
import { get } from "svelte/store";
import { root_traces } from "./root_store";

export function get_root_traces_store(): Readable<Trace[] | null> {
    return root_traces;
}

export function get_root_traces(): Trace[] | null {
    return get(root_traces);
}
