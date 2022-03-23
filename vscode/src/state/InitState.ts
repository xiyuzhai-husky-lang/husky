import { decode_array } from "src/cast/cast";
import Trace from "src/trace/Trace";

export default class InitState {
    active_trace_id: number | null;
    opt_input_id: number | null;
    traces: Trace[];
    subtraces_list: [[number, number | null], number[]][];
    expansions: { [id_str: string]: boolean };
    showns: { [id_str: string]: boolean };
    root_traces: number[];
    constructor(props: any) {
        this.active_trace_id = props.active_trace_id;
        this.opt_input_id = props.opt_input_id;
        this.traces = decode_array(
            props.traces,
            (element) => new Trace(element)
        );
        this.subtraces_list = props.subtraces_list;
        this.expansions = props.expansions;
        this.showns = props.showns;
        this.root_traces = props.root_traces;
    }
}

export function decode_init_state(props: unknown) {
    return new InitState(props);
}
