import type FigureProps from "trace/figure/FigureProps";
import InitState, { decode_init_state } from "src/state/InitState";
import Trace, { decode_trace } from "src/trace/Trace";
import type TraceStalk from "src/trace/stalk/TraceStalk";
import {
    decode_array,
    decode_member,
    decode_number,
    decode_number_or_null,
    decode_number_or_null_or_undefined,
    decode_or_null,
    decode_string,
} from "src/cast/cast";
import { decode_figure_props } from "trace/figure/FigureProps";

export type InitMessage = {
    type: "Init";
    init_state: InitState;
};
type SubtracesMessage = {
    type: "Subtraces";
    id: number;
    input_locked_on: number | null;
    subtraces: Trace[];
};
type TraceMessage = {
    type: "Trace";
    trace: Trace;
};
type FigureMessage = {
    type: "Figure";
    id: number;
    figure: FigureProps | null;
};
type DidActivateMessage = {
    type: "DidActivate";
    id: number;
};
type DidToggleExpansionMessage = {
    type: "DidToggleExpansion";
    id: number;
};
type DidToggleShowMessage = {
    type: "DidToggleShow";
    id: number;
};
type DidLockInputMessage = {
    type: "DidLockInput";
    input_locked_on: number | null | undefined;
    message: string | null;
};
type SetShownMessage = {
    type: "SetShown";
    trace_id: number;
    is_shown: boolean;
};
export type TraceStalkMessage = {
    type: "TraceStalk";
    trace_id: number;
    input_id: number;
    stalk: TraceStalk;
};
export type DebuggerMessage =
    | InitMessage
    | SubtracesMessage
    | TraceMessage
    | TraceStalkMessage
    | FigureMessage
    | DidActivateMessage
    | DidToggleExpansionMessage
    | DidToggleShowMessage
    | DidLockInputMessage
    | SetShownMessage;

export default DebuggerMessage;

export function parse_debugger_message(text: string): DebuggerMessage {
    let props: unknown = JSON.parse(text);
    let type = decode_member(props, "type", decode_string);
    switch (decode_string(type)) {
        case "Init":
            return {
                type: "Init",
                init_state: decode_member(
                    props,
                    "init_state",
                    decode_init_state
                ),
            };
        case "Subtraces":
            return {
                type: "Subtraces",
                id: decode_member(props, "id", decode_number),
                input_locked_on: decode_member(
                    props,
                    "input_locked_on",
                    decode_number_or_null
                ),
                subtraces: decode_member(props, "subtraces", (subtraces) =>
                    decode_array(subtraces, (element) => new Trace(element))
                ),
            };
        case "Trace":
            return {
                type: "Trace",
                trace: decode_member(props, "trace", decode_trace),
            };
        case "Figure":
            return {
                type: "Figure",
                id: decode_member(props, "id", decode_number),
                figure: decode_member(props, "figure", (data) =>
                    decode_or_null(data, (data) => decode_figure_props(data))
                ),
            };
        case "DidActivate":
            return {
                type: "DidActivate",
                id: decode_member(props, "id", decode_number),
            };
        case "DidToggleExpansion":
            return {
                type: "DidToggleExpansion",
                id: decode_member(props, "id", decode_number),
            };
        case "DidToggleShow":
            return {
                type: "DidToggleShow",
                id: decode_member(props, "id", decode_number),
            };
        case "DidLockInput":
            return {
                type: "DidLockInput",
                input_locked_on: decode_member(
                    props,
                    "input_locked_on",
                    decode_number_or_null_or_undefined
                ),
                message: decode_member(props, "message", decode_string),
            };
        //     return new SubtracesMessage(props);
        // case "TraceMessage":
        //     return new TraceMessage(props);
        case "TraceStalk":
            return {
                type: "TraceStalk",
                trace_id: decode_member(props, "trace_id", decode_number),
                input_id: decode_member(props, "input_id", decode_number),
                stalk: decode_member(
                    props,
                    "stalk",
                    (data: unknown) => data as TraceStalk
                ),
            };
        default:
            throw new Error(`unhandled Debugger Message type "${type}"`);
    }

    // function  new Message: any, data: any): any {
    //     let result = tMessage.decode(data);
    //     if (!isRight(result)) {
    //         throw new Error(`init_report: ${reporter.report(result)}`);
    //     }
    //     return data;
    // }
}
