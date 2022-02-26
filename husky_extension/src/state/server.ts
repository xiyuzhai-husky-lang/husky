import type FigureProps from "src/trace/figure/FigureProps";
import type TraceStalk from "src/trace/stalk/TraceStalk";
import type Trace from "src/trace/Trace";
import global_state from "./global_state";
import type RawState from "./State/InitState";

export function receive_init_message(raw_state: RawState) {
    global_state.init(raw_state);
}

export function receive_subtraces(
    trace_id: number,
    input_id: number | null,
    subtraces: Trace[]
) {
    global_state.trace_cache.receive_subtraces(trace_id, input_id, subtraces);
    global_state.update_trace_listing();
}

export function cache_trace(trace: Trace) {
    global_state.trace_cache.cache_trace(trace);
    global_state.update_trace_listing();
}

export function set_trace_stalk(
    trace_id: number,
    input_id: number,
    stalk: TraceStalk
) {
    global_state.trace_cache.set_trace_stalk(trace_id, input_id, stalk);
}

export function set_figure(id: number, figure: FigureProps) {
    global_state.figure_state.set_figure(
        id,
        figure,
        global_state.user_state.active_trace()
    );
}

export function did_activate(trace_id: number) {
    let trace = global_state.trace_cache.get_trace(trace_id);
    global_state.user_state.active_trace_store.set(trace);
}

export function did_toggle_expansion(trace_id: number) {
    global_state.user_state.did_toggle_expansion(trace_id);
    global_state.update_trace_listing();
}

export function did_toggle_show(trace_id: number) {
    global_state.user_state.did_toggle_show(trace_id);
    global_state.update_trace_listing();
}

export function did_lock_input(
    input_locked_on: number | null | undefined,
    message: string | null
) {
    global_state.user_state.did_lock_input(input_locked_on, message);
}
