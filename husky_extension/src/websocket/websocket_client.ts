import { pending_requests } from "./pending";
import { websocket } from "./websocket";

function try_send_request(request: any) {
    switch (websocket.readyState) {
        case 0:
            // CONNECTING
            pending_requests.push(request);
            break;
        case 1:
            // OPEN
            websocket.send(JSON.stringify(request));
            break;
        case 2:
            // CLOSING
            break;
        case 3:
            // CLOSED
            break;
    }
}

export function request_subtraces(
    trace_id: number,
    opt_input_id: number | null
) {
    try_send_request({
        type: "Subtraces",
        trace_id,
        opt_input_id,
    });
}

export function request_toggle_expansion(id: number) {
    try_send_request({ type: "ToggleExpansion", id });
}

export function request_toggle_show(id: number) {
    try_send_request({ type: "ToggleShow", id });
}

export function request_activate(id: number, figure_cached: boolean) {
    prepare_figure(id, figure_cached);
    try_send_request({ type: "Activate", id });

    function prepare_figure(id: number, figure_cached: boolean) {
        if (!figure_cached) {
            try_send_request({ type: "Figure", id });
        }
    }
}

export function request_trace(id: number) {
    try_send_request({ type: "Trace", id });
}

export function request_lock_input(input_str: number | null) {
    try_send_request({ type: "LockInput", input_str });
}

export function request_trace_stalk(trace_id: number, input_id: number) {
    try_send_request({ type: "TraceStalk", trace_id, input_id });
}
