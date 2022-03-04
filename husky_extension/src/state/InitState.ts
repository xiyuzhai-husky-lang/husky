import * as t from "io-ts";
import { tTrace } from "src/trace/Trace";

export const tInitState = t.type({
    active_trace_id: t.union([t.number, t.null]),
    opt_input_id: t.union([t.number, t.null]),
    traces: t.array(tTrace),
    subtraces_list: t.array(
        t.tuple([
            t.tuple([t.number, t.union([t.number, t.null])]),
            t.array(t.number),
        ])
    ),
    root_traces: t.array(t.number),
    expansions: t.record(t.string, t.boolean),
    showns: t.record(t.string, t.boolean),
});
type RawState = t.TypeOf<typeof tInitState>;
export default RawState;
