import {
    decode_array,
    decode_memb,
    decode_number,
    decode_opt,
} from "src/decode/decode";
import type FigureControlProps from "src/trace/figure/FigureControlProps";
import type FigureProps from "src/trace/figure/FigureProps";
import Trace from "src/trace/Trace";
import Focus from "../Focus";

export default class InitData {
    active_trace_id: number | null;
    traces: Trace[];
    subtraces_list: [[number, number | null], number[]][];
    expansions: { [id_str: string]: boolean };
    showns: { [id_str: string]: boolean };
    root_traces: number[];
    focus: Focus;
    figures: { [id_str: string]: FigureProps };
    figure_controls: { [id_str: string]: FigureControlProps };
    constructor(props: unknown) {
        this.active_trace_id = decode_opt(
            decode_memb(props, "active_trace_id"),
            decode_number
        );
        this.focus = new Focus(decode_memb(props, "focus"));
        this.traces = decode_array(
            decode_memb(props, "traces"),
            (element) => new Trace(element)
        );
        this.subtraces_list = decode_memb(props, "subtraces_list") as any;
        this.expansions = decode_memb(props, "expansions") as any;
        this.showns = decode_memb(props, "showns") as any;
        this.root_traces = decode_array(
            decode_memb(props, "root_traces"),
            decode_number
        );
        this.figures = decode_memb(props, "figures") as any;
        this.figure_controls = decode_memb(props, "figure_controls") as any;
    }
}

export function decode_init_state(props: unknown) {
    return new InitData(props);
}
