import { has_figure } from "src/trace/figure/figure_server";
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

export function request_root_traces() {
    try_send_request({ type: "RootTraces" });
}

export function request_subtraces(id: number) {
    try_send_request({ type: "Subtraces", id });
}

export function request_toggle_expansion(id: number) {
    try_send_request({ type: "ToggleExpansion", id });
}

export function request_toggle_show(id: number) {
    try_send_request({ type: "ToggleShow", id });
}

export function request_activate(id: number) {
    prepare_figure(id);
    try_send_request({ type: "Activate", id });

    function prepare_figure(id: number) {
        if (!has_figure(id)) {
            try_send_request({ type: "Figure", id });
        }
    }
}

export function request_trace(id: number) {
    try_send_request({ type: "Trace", id });
}

export function request_lock_input(input_temp: number | null) {
    try_send_request({ type: "LockInput", input_temp });
}
