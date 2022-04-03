import type FigureProps from "trace/figure/FigureProps";
import InitData, { decode_init_state } from "src/data/InitData";
import Trace, { decode_trace } from "src/trace/Trace";
import type TraceStalk from "src/trace/stalk/TraceStalk";
import {
    decode_array,
    decode_member,
    decode_member_old,
    decode_number,
    decode_opt,
    decode_string,
} from "src/decode/decode";
import { decode_figure_props } from "trace/figure/FigureProps";
import Focus from "src/data/Focus";
import { decode_result } from "src/abstraction/Result";
import type Result from "src/abstraction/Result";

export type InitResponse = {
    kind: "Init";
    init_state: InitData;
};
type TraceResponse = {
    kind: "Trace";
    trace: Trace;
};
type ActivateResponse = {
    kind: "Activate";
    id: number;
    opt_focus_for_figure: Focus | null;
    opt_figure: FigureProps | null;
};
type ToggleExpansionResponse = {
    kind: "ToggleExpansion";
    id: number;
    effective_opt_input_id: number | null;
    opt_subtraces: Trace[] | null;
};

type ToggleShowResponse = {
    kind: "ToggleShow";
    id: number;
};

type DecodeFocusResponse = {
    kind: "DecodeFocus";
    focus_result: Result<Focus>;
};

export type LockFocusResponse = {
    kind: "LockFocus";
    focus: Focus;
    opt_active_trace_id_for_figure: number | null;
    opt_figure: FigureProps | null;
};

type SetShownResponse = {
    kind: "SetShown";
    trace_id: number;
    is_shown: boolean;
};
export type TraceStalkResponse = {
    kind: "TraceStalk";
    trace_id: number;
    input_id: number;
    stalk: TraceStalk;
};
export type DebuggerResponse =
    | InitResponse
    | TraceResponse
    | TraceStalkResponse
    | ActivateResponse
    | ToggleExpansionResponse
    | ToggleShowResponse
    | DecodeFocusResponse
    | LockFocusResponse
    | SetShownResponse;

export default DebuggerResponse;

export function parse_debugger_response(text: string): DebuggerResponse {
    let props: unknown = JSON.parse(text);
    let type = decode_member_old(props, "kind", decode_string);
    switch (decode_string(type)) {
        case "Init":
            return {
                kind: "Init",
                init_state: decode_member_old(
                    props,
                    "init_state",
                    decode_init_state
                ),
            };
        case "Trace":
            return {
                kind: "Trace",
                trace: decode_member_old(props, "trace", decode_trace),
            };
        case "Activate":
            return {
                kind: "Activate",
                id: decode_member_old(props, "id", decode_number),
                opt_focus_for_figure: decode_opt(
                    decode_member(props, "opt_focus_for_figure"),
                    (data: unknown) => new Focus(data)
                ),
                opt_figure: decode_opt(
                    decode_member(props, "opt_figure"),
                    decode_figure_props
                ),
            };
        case "ToggleExpansion":
            return {
                kind: "ToggleExpansion",
                id: decode_member_old(props, "id", decode_number),
                effective_opt_input_id: decode_opt(
                    decode_member(props, "effective_opt_input_id"),
                    decode_number
                ),
                opt_subtraces: decode_opt(
                    decode_member(props, "opt_subtraces"),
                    (data: unknown) =>
                        decode_array(data, (data: unknown) => new Trace(data))
                ),
            };
        case "ToggleShow":
            return {
                kind: "ToggleShow",
                id: decode_member_old(props, "id", decode_number),
            };
        case "DecodeFocus":
            return {
                kind: "DecodeFocus",
                focus_result: decode_result(
                    decode_member(props, "focus_result"),
                    (data: unknown) => new Focus(data)
                ),
            };
        case "LockFocus":
            return {
                kind: "LockFocus",
                focus: new Focus(decode_member(props, "focus")),
                opt_active_trace_id_for_figure: decode_opt(
                    decode_member(props, "opt_active_trace_id_for_figure"),
                    decode_number
                ),
                opt_figure: decode_opt(
                    decode_member(props, "opt_figure"),
                    decode_figure_props
                ),
            };
        case "TraceStalk":
            return {
                kind: "TraceStalk",
                trace_id: decode_member_old(props, "trace_id", decode_number),
                input_id: decode_member_old(props, "input_id", decode_number),
                stalk: decode_member_old(
                    props,
                    "stalk",
                    (data: unknown) => data as TraceStalk
                ),
            };
        default:
            throw new Error(`unhandled Debugger Response type "${type}"`);
    }
}
