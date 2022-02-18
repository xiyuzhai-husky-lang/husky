import type Trace from "../Trace";
import type { Writable } from "svelte/store";

export let subtraces_stores: { [trace_id: number]: Writable<Trace[] | null> } =
    {};
export let subtraces_locked_on_stores_table: {
    [trace_id: number]: { [input_id: number]: Writable<Trace[] | null> };
} = {};
