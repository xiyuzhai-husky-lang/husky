import type DebuggerResponse from "./DebuggerResponse";
import * as server_handle from "src/data/server_handle";
import { parse_debugger_response } from "./DebuggerResponse";
import type Focus from "src/data/Focus";

class Server {
    websocket = new WebSocket("ws://localhost:51617/query");
    pending_requests: any[] = [];

    constructor() {
        this.init_websocket();
    }

    init_websocket() {
        this.websocket.addEventListener("open", (event: Event) => {
            server_instance.send_pending_requests();
        });
        this.websocket.addEventListener(
            "message",
            function (event: MessageEvent) {
                let data: DebuggerResponse = parse_debugger_response(
                    event.data
                );
                server_instance.handle_response(data);
            }
        );
    }

    handle_response(response: DebuggerResponse) {
        switch (response.kind) {
            case "Init":
                server_handle.receive_init_response(response.init_state);
                break;
            case "Trace":
                server_handle.cache_trace(response.trace);
                break;
            case "TraceStalk":
                server_handle.set_trace_stalk(
                    response.trace_id,
                    response.input_id,
                    response.stalk
                );
                break;
            case "Activate":
                server_handle.did_activate(
                    response.id,
                    response.opt_focus_for_figure,
                    response.opt_figure
                );
                break;
            case "ToggleExpansion":
                console.log("ToggleExpansionResponse: ", response);
                if (response.opt_subtraces !== null) {
                    server_handle.receive_subtraces(
                        response.id,
                        response.effective_opt_input_id,
                        response.opt_subtraces
                    );
                }
                server_handle.did_toggle_expansion(response.id);
                break;
            case "ToggleShow":
                server_handle.did_toggle_show(response.id);
                break;
            case "DecodeFocus":
                switch (response.focus_result.kind) {
                    case "Ok":
                        const focus = response.focus_result.value;
                        server_instance.try_send_request({
                            kind: "LockFocus",
                            focus,
                            opt_active_trace_id_for_figure:
                                server_handle.opt_active_trace_id_for_figure(
                                    focus
                                ),
                        });
                        break;
                    case "Err":
                        alert!(response.focus_result.message);
                        break;
                }
                break;
            case "LockFocus":
                server_handle.did_lock_focus(response);
                break;
        }
    }

    send_pending_requests() {
        for (const request of this.pending_requests) {
            this.websocket.send(JSON.stringify(request));
        }
        this.pending_requests = [];
    }

    try_send_request(request: any) {
        switch (server_instance.websocket.readyState) {
            case 0:
                // CONNECTING
                this.pending_requests.push(request);
                break;
            case 1:
                // OPEN
                this.websocket.send(JSON.stringify(request));
                break;
            case 2:
                // CLOSING
                break;
            case 3:
                // CLOSED
                break;
        }
    }
}

const server_instance = new Server();

export function request_subtraces(
    trace_id: number,
    opt_input_id: number | null
) {
    server_instance.try_send_request({
        kind: "Subtraces",
        trace_id,
        opt_input_id,
    });
}

export function request_toggle_expansion(
    id: number,
    request_subtraces: boolean
) {
    console.log("request toggle expansion", {
        kind: "ToggleExpansion",
        id,
        request_subtraces,
    });
    server_instance.try_send_request({
        kind: "ToggleExpansion",
        id,
        request_subtraces,
    });
}

export function request_toggle_show(id: number) {
    server_instance.try_send_request({ kind: "ToggleShow", id });
}

export function request_activate(
    id: number,
    opt_focus_for_figure: Focus | null
) {
    server_instance.try_send_request({
        kind: "Activate",
        id,
        opt_focus_for_figure,
    });
}

export function request_trace(id: number) {
    server_instance.try_send_request({ kind: "Trace", id });
}

export function request_lock_input(command: string) {
    server_instance.try_send_request({
        kind: "DecodeFocus",
        command,
    });
}

export function request_trace_stalk(trace_id: number, input_id: number) {
    server_instance.try_send_request({
        kind: "TraceStalk",
        trace_id,
        input_id,
    });
}

export function request_figure(trace_id: number, opt_input_id: number | null) {
    server_instance.try_send_request({
        kind: "Figure",
        trace_id,
        opt_input_id,
    });
}
