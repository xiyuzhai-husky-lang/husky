import type FigureProps from "trace/figure/FigureProps";
import InitData, { decode_init_state } from "src/data/InitData";
import Trace, { decode_trace } from "src/trace/Trace";
import type TraceStalk from "src/trace/stalk/TraceStalk";
import {
    decode_array,
    d_memb,
    d_memb_old,
    decode_number,
    decode_opt,
    d_string,
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
    associated_traces: Trace[];
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
    let type = d_memb_old(props, "kind", d_string);
    switch (d_string(type)) {
        case "Init":
            return {
                kind: "Init",
                init_state: d_memb_old(props, "init_state", decode_init_state),
            };
        case "Trace":
            return {
                kind: "Trace",
                trace: d_memb_old(props, "trace", decode_trace),
            };
        case "Activate":
            return {
                kind: "Activate",
                id: d_memb_old(props, "id", decode_number),
                opt_focus_for_figure: decode_opt(
                    d_memb(props, "opt_focus_for_figure"),
                    (data: unknown) => new Focus(data)
                ),
                opt_figure: decode_opt(
                    d_memb(props, "opt_figure"),
                    decode_figure_props
                ),
            };
        case "ToggleExpansion":
            return {
                kind: "ToggleExpansion",
                id: d_memb_old(props, "id", decode_number),
                effective_opt_input_id: decode_opt(
                    d_memb(props, "effective_opt_input_id"),
                    decode_number
                ),
                opt_subtraces: decode_opt(
                    d_memb(props, "opt_subtraces"),
                    (data: unknown) => decode_array(data, decode_trace)
                ),
                associated_traces: decode_array(
                    d_memb(props, "associated_traces"),
                    decode_trace
                ),
            };
        case "ToggleShow":
            return {
                kind: "ToggleShow",
                id: d_memb_old(props, "id", decode_number),
            };
        case "DecodeFocus":
            return {
                kind: "DecodeFocus",
                focus_result: decode_result(
                    d_memb(props, "focus_result"),
                    (data: unknown) => new Focus(data)
                ),
            };
        case "LockFocus":
            return {
                kind: "LockFocus",
                focus: new Focus(d_memb(props, "focus")),
                opt_active_trace_id_for_figure: decode_opt(
                    d_memb(props, "opt_active_trace_id_for_figure"),
                    decode_number
                ),
                opt_figure: decode_opt(
                    d_memb(props, "opt_figure"),
                    decode_figure_props
                ),
            };
        case "TraceStalk":
            return {
                kind: "TraceStalk",
                trace_id: d_memb_old(props, "trace_id", decode_number),
                input_id: d_memb_old(props, "input_id", decode_number),
                stalk: d_memb_old(
                    props,
                    "stalk",
                    (data: unknown) => data as TraceStalk
                ),
            };
        default:
            throw new Error(`unhandled Debugger Response type "${type}"`);
    }
}
