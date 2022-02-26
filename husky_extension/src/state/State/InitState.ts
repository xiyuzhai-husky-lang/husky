import * as t from "io-ts";
import { tTrace } from "src/trace/Trace";

export const tRawState = t.type({
    active_trace_id: t.union([t.number, t.null]),
    opt_input_id: t.union([t.number, t.null]),
    traces: t.array(tTrace),
    subtraces: t.record(t.string, t.array(t.number)),
    root_traces: t.array(t.number),
    expansions: t.record(t.string, t.boolean),
    showns: t.record(t.string, t.boolean),
});
type RawState = t.TypeOf<typeof tRawState>;
export default RawState;
