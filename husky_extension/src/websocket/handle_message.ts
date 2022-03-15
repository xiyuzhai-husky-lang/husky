import type DebuggerMessage from "./DebuggerMessage";
import * as state from "src/state/server";

export function handle_message(message: DebuggerMessage) {
    switch (message.type) {
        case "Init":
            state.receive_init_message(message.init_state);
            break;
        case "Subtraces":
            state.receive_subtraces(
                message.id,
                message.input_locked_on,
                message.subtraces
            );
            break;
        case "Trace":
            state.cache_trace(message.trace);
            break;
        case "TraceStalk":
            state.set_trace_stalk(
                message.trace_id,
                message.input_id,
                message.stalk
            );
            break;
        case "Figure":
            state.set_figure(message.id, message.figure);
            break;
        case "DidActivate":
            state.did_activate(message.id);
            break;
        case "DidToggleExpansion":
            state.did_toggle_expansion(message.id);
            break;
        case "DidToggleShow":
            state.did_toggle_show(message.id);
            break;
        case "DidLockInput":
            state.did_lock_input(message.input_locked_on, message.message);
            break;
    }
}
