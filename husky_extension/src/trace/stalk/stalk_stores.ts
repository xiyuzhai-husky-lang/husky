import type { Writable } from "svelte/store";
import type TraceStalk from "./TraceStalk";

export let trace_stalk_stores_table: {
    [trace_id: number]: { [input_id: number]: Writable<TraceStalk | null> };
} = {};
