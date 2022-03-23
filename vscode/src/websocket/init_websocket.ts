import type DebuggerMessage from "./DebuggerMessage";
import { parse_debugger_message } from "./DebuggerMessage";
import { send_pending_requests } from "./pending";
import { handle_message } from "./handle_message";

export function init_websocket(websocket: WebSocket) {
    websocket.addEventListener("open", (event: Event) => {
        send_pending_requests(websocket);
    });
    websocket.addEventListener("message", function (event: MessageEvent) {
        let data: DebuggerMessage = parse_debugger_message(event.data);
        handle_message(data);
    });
}
