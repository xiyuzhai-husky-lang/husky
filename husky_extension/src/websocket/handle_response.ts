import type DebuggerResponse from "./DebuggerResponse";
import { set_root_traces } from "src/trace/root/root_server";
import { set_subtraces } from "src/trace/subtraces/server";
import {
    did_toggle_expansion,
    did_toggle_show,
} from "src/trace/status/status_server";
import { set_figure } from "src/trace/figure/figure_server";
import { did_activate } from "src/trace/activate/server";
import { cache_trace } from "trace/trace_server";
import { update_trace_listing } from "src/trace/listing/listing_server";
import { did_lock_input } from "src/trace/input/input_server";

export function handle_response(response: DebuggerResponse) {
    switch (response.type) {
        case "RootTraces":
            set_root_traces(response.root_traces);
            update_trace_listing();
            break;
        case "Subtraces":
            set_subtraces(response.id, response.subtraces);
            update_trace_listing();
            break;
        case "Trace":
            cache_trace(response.trace);
            update_trace_listing();
            break;
        case "Figure":
            set_figure(response.id, response.figure);
            break;
        case "DidActivate":
            did_activate(response.id);
            break;
        case "DidToggleExpansion":
            did_toggle_expansion(response.id);
            break;
        case "DidToggleShow":
            did_toggle_show(response.id);
            break;
        case "DidLockInput":
            did_lock_input(response.input_locked_on, response.message);
            break;
    }
}
