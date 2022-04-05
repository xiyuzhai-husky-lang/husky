import {
    decode_array,
    decode_member,
    decode_number,
    decode_opt,
} from "src/decode/decode";
import type FigureProps from "src/trace/figure/FigureProps";
import Trace from "src/trace/Trace";
import Focus from "./Focus";

export default class InitData {
    active_trace_id: number | null;
    traces: Trace[];
    subtraces_list: [[number, number | null], number[]][];
    expansions: { [id_str: string]: boolean };
    showns: { [id_str: string]: boolean };
    root_traces: number[];
    focus: Focus;
    figures: { [id_str: string]: FigureProps };
    constructor(props: unknown) {
        this.active_trace_id = decode_opt(
            decode_member(props, "active_trace_id"),
            decode_number
        );
        console.log("props.focus", decode_member(props, "focus"));
        this.focus = new Focus(decode_member(props, "focus"));
        console.log("this.focus", this.focus);
        this.traces = decode_array(
            decode_member(props, "traces"),
            (element) => new Trace(element)
        );
        this.subtraces_list = decode_member(props, "subtraces_list") as any;
        this.expansions = decode_member(props, "expansions") as any;
        this.showns = decode_member(props, "showns") as any;
        this.root_traces = decode_array(
            decode_member(props, "root_traces"),
            decode_number
        );
        this.figures = decode_member(props, "figures") as any;
    }
}

export function decode_init_state(props: unknown) {
    return new InitData(props);
}
