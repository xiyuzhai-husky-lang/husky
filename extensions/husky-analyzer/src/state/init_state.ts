import {
    decode_array,
    decode_memb,
    decode_number,
    decode_opt,
} from "src/decode/decode";
import type { FigureControlProps } from "src/figure/control";
import type FigureProps from "src/figure";
import { Trace } from "src/trace";
import { Focus } from "src/focus";
import type Dict from "src/abstraction/Dict";
import { decode_figure_control_props } from "src/figure/control";
import { decode_dict } from "src/abstraction/Dict";

export class InitState {
    active_trace_id: number | null;
    traces: Trace[];
    subtraces_list: [[number, number | null], number[]][];
    expansions: { [id_str: string]: boolean };
    showns: { [id_str: string]: boolean };
    root_traces: number[];
    focus: Focus;
    figures: { [id_str: string]: FigureProps };
    figure_controls: Dict<FigureControlProps>;
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
        this.expansions = decode_memb(raw_init_state, "expansions") as any;
        this.showns = decode_memb(raw_init_state, "showns") as any;
        this.root_traces = decode_array(
            decode_memb(raw_init_state, "root_traces"),
            decode_number
        );
        this.figures = decode_memb(raw_init_state, "figures") as any;
        this.figure_controls = decode_dict(
            decode_memb(raw_init_state, "figure_controls"),
            decode_figure_control_props
        );
    }
}

export function decode_init_state(response_variant: unknown) {
    return new InitState(response_variant);
}
