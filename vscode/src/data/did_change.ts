import type FigureProps from "src/trace/figure/FigureProps";
import type TraceStalk from "src/trace/stalk/TraceStalk";
import type Trace from "src/trace/Trace";
import type Focus from "./Focus";
import global from "./global";
import type InitData from "./InitData";

export function receive_init_response(init_state: InitData) {
    global.init(init_state);
}

export function receive_subtraces(
    trace_id: number,
    eff_opt_input_id: number | null,
    subtraces: Trace[]
) {
    console.log("receive subtraces for trace id = ", trace_id);
    global.trace_cache.receive_subtraces(trace_id, eff_opt_input_id, subtraces);
    global.update_trace_listing();
}

export function cache_trace(trace: Trace) {
    global.trace_cache.cache_trace(trace);
    global.update_trace_listing();
}

export function set_trace_stalk(
    trace_id: number,
    input_id: number,
    stalk: TraceStalk
) {
    global.trace_cache.set_trace_stalk(trace_id, input_id, stalk);
}

export function set_figure(
    trace_id: number,
    focus: Focus,
    figure: FigureProps
) {
    global.figure_cache.set_figure(trace_id, focus, figure);
}

export function did_activate(
    trace_id: number,
    opt_focus_for_figure: Focus | null,
    opt_figure: FigureProps | null
) {
    if (opt_figure !== null) {
        if (opt_focus_for_figure === null) {
            throw new Error("panic");
        }
        set_figure(trace_id, opt_focus_for_figure, opt_figure);
    }
    let trace = global.trace_cache.get_trace(trace_id);
    global.user_state.active_trace_store.set(trace);
}

export function did_toggle_expansion(trace_id: number) {
    global.user_state.did_toggle_expansion(trace_id);
    global.update_trace_listing();
}

export function did_toggle_show(trace_id: number) {
    global.user_state.did_toggle_show(trace_id);
    global.update_trace_listing();
}

export function did_lock_focus(focus: Focus) {
    global.user_state.did_lock_focus(focus);
}
