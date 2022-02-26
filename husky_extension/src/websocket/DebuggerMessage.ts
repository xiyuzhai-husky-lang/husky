import * as t from "io-ts";

import { tFigureProps } from "trace/figure/FigureProps";
import { tTrace } from "src/trace/Trace";
import { isRight } from "fp-ts/Either";
import { tTraceStalk } from "src/trace/stalk/TraceStalk";
import { tInitState } from "src/state/State/InitState";
import reporter from "io-ts-reporters";

export const tInitMessage = t.interface({
    type: t.literal("Init"),
    raw_state: tInitState,
});
const tSubtracesMessage = t.interface({
    type: t.literal("Subtraces"),
    id: t.number,
    input_locked_on: t.union([t.number, t.null]),
    subtraces: t.array(tTrace),
});
const tTraceMessage = t.interface({
    type: t.literal("Trace"),
    trace: tTrace,
});
const tFigureMessage = t.interface({
    type: t.literal("Figure"),
    id: t.number,
    figure: t.union([tFigureProps, t.null]),
});
const tDidActivateMessage = t.interface({
    type: t.literal("DidActivate"),
    id: t.number,
});
const tDidToggleExpansionMessage = t.interface({
    type: t.literal("DidToggleExpansion"),
    id: t.number,
});
const tDidToggleShowMessage = t.interface({
    type: t.literal("DidToggleShow"),
    id: t.number,
});
const tDidLockInputMessage = t.interface({
    type: t.literal("DidLockInput"),
    input_locked_on: t.union([t.number, t.null, t.undefined]),
    message: t.union([t.string, t.null]),
});
const tSetShownMessage = t.interface({
    type: t.literal("SetShown"),
    trace_id: t.number,
    is_shown: t.boolean,
});
export const tTraceStalkMessage = t.interface({
    type: t.literal("TraceStalk"),
    trace_id: t.number,
    input_id: t.number,
    stalk: tTraceStalk,
});
export const tDebuggerMessage = t.union([
    tInitMessage,
    tSubtracesMessage,
    tTraceMessage,
    tTraceStalkMessage,
    tFigureMessage,
    tDidActivateMessage,
    tDidToggleExpansionMessage,
    tDidToggleShowMessage,
    tDidLockInputMessage,
    tSetShownMessage,
]);

type DebuggerMessage = t.TypeOf<typeof tDebuggerMessage>;
export default DebuggerMessage;

export function parse_debugger_message(text: string): DebuggerMessage {
    let data = JSON.parse(text);
    if (typeof data.type !== "string") {
        throw new Error(
            `In parsing debugger message, data.type !== \"string\" but is ${data.type} instead`
        );
    }
    switch (data.type) {
        case "Init":
            return parse(tInitMessage, data);
        case "Trace":
            return parse(tTraceMessage, data);
        case "Subtraces":
            return parse(tSubtracesMessage, data);
        default:
            return parse(tDebuggerMessage, data);
    }

    function parse(tMessage: any, data: any): any {
        let result = tMessage.decode(data);
        if (!isRight(result)) {
            throw new Error(`init_report: ${reporter.report(result)}`);
        }
        return data;
    }
}
