import { trace_stalk_stores_table } from "./stalk_stores";
import type TraceStalk from "./TraceStalk";

export function set_trace_stalk(
    trace_id: number,
    input_id: number,
    stalk: TraceStalk
) {
    trace_stalk_stores_table[trace_id][input_id].set(stalk);
}
