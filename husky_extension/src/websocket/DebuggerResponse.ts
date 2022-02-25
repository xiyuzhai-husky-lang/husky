import * as t from "io-ts";

import { tFigureProps } from "trace/figure/FigureProps";
import { tTrace } from "src/trace/Trace";
import { tTraceStalk } from "src/trace/stalk/TraceStalk";
export const tInitResponse = t.interface({
    type: t.literal("Init"),
    active_trace_id: t.union([t.number, t.null]),
    opt_input_id: t.union([t.number, t.null]),
    root_traces: t.array(tTrace),
    expansions: t.record(t.string, t.boolean),
    showns: t.record(t.string, t.boolean),
});
const tSubtracesResponse = t.interface({
    type: t.literal("Subtraces"),
    id: t.number,
    input_locked_on: t.union([t.number, t.null]),
    subtraces: t.array(tTrace),
});
const tTraceResponse = t.interface({
    type: t.literal("Trace"),
    trace: tTrace,
});
const tFigureResponse = t.interface({
    type: t.literal("Figure"),
    id: t.number,
    figure: t.union([tFigureProps, t.null]),
});
const tDidActivateResponse = t.interface({
    type: t.literal("DidActivate"),
    id: t.number,
});
const tDidToggleExpansionResponse = t.interface({
    type: t.literal("DidToggleExpansion"),
    id: t.number,
});
const tDidToggleShowResponse = t.interface({
    type: t.literal("DidToggleShow"),
    id: t.number,
});
const tDidLockInputResponse = t.interface({
    type: t.literal("DidLockInput"),
    input_locked_on: t.union([t.number, t.null, t.undefined]),
    message: t.union([t.string, t.null]),
});
const tSetShownResponse = t.interface({
    type: t.literal("SetShown"),
    trace_id: t.number,
    is_shown: t.boolean,
});
export const tTraceStalkResponse = t.interface({
    type: t.literal("TraceStalk"),
    trace_id: t.number,
    input_id: t.number,
    stalk: tTraceStalk,
});
export const tDebuggerResponse = t.union([
    tInitResponse,
    tSubtracesResponse,
    tTraceResponse,
    tTraceStalkResponse,
    tFigureResponse,
    tDidActivateResponse,
    tDidToggleExpansionResponse,
    tDidToggleShowResponse,
    tDidLockInputResponse,
    tSetShownResponse,
]);

type DebuggerResponse = t.TypeOf<typeof tDebuggerResponse>;
export default DebuggerResponse;
