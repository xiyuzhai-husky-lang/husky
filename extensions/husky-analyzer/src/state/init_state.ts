import {
    decode_array,
    decode_boolean,
    decode_memb,
    decode_number,
    decode_opt,
} from "src/decode/decode";
import type { FigureControlData } from "src/figure/control";
import type FigureContentProps from "src/figure";
import { Trace } from "src/trace";
import { Focus } from "src/focus";
import type Dict from "src/abstraction/Dict";
import { decode_figure_control_props } from "src/figure/control";
import { decode_dict } from "src/abstraction/Dict";
import { decode_figure_props } from "src/figure";

export class InitState {
    active_trace_id: number | null;
    traces: Trace[];
    subtraces_list: [[number, number | null], number[]][];
    expansions: Dict<boolean>;
    showns: Dict<boolean>;
    root_traces: number[];
    focus: Focus;
    figures: Dict<FigureContentProps>;
    figure_controls: Dict<FigureControlData>;
    constructor(response_variant: unknown) {
        const raw_init_state = decode_memb(response_variant, "init_state");
        this.active_trace_id = decode_opt(
            decode_memb(raw_init_state, "active_trace_id"),
            decode_number
        );
        this.focus = new Focus(decode_memb(raw_init_state, "focus"));
        this.traces = decode_array(
            decode_memb(raw_init_state, "traces"),
            (element) => new Trace(element)
        );
        this.subtraces_list = decode_memb(
            raw_init_state,
            "subtraces_list"
        ) as any;
        this.expansions = decode_dict(
            decode_memb(raw_init_state, "expansions"),
            decode_boolean
        );
        this.showns = decode_dict(
            decode_memb(raw_init_state, "showns"),
            decode_boolean
        );
        this.root_traces = decode_array(
            decode_memb(raw_init_state, "root_traces"),
            decode_number
        );
        this.figures = decode_dict(
            decode_memb(raw_init_state, "figures"),
            decode_figure_props
        );
        this.figure_controls = decode_dict(
            decode_memb(raw_init_state, "figure_controls"),
            decode_figure_control_props
        );
    }
}

export function decode_init_state(response_variant: unknown) {
    return new InitState(response_variant);
}
