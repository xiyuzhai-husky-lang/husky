import type DebuggerResponse from "./DebuggerResponse";
import { tDebuggerResponse } from "./DebuggerResponse";
import { isRight } from "fp-ts/Either";
import { PathReporter } from "io-ts/PathReporter";
import { send_pending_requests } from "./pending";
import { handle_response } from "./handle_response";

export function init_websocket(websocket: WebSocket) {
    websocket.addEventListener("open", (event: Event) => {
        send_pending_requests(websocket);
    });
    websocket.addEventListener("message", function (event: MessageEvent) {
        let data: DebuggerResponse = JSON.parse(event.data);
        const result = tDebuggerResponse.decode(data);
        if (isRight(result)) {
            handle_response(data);
        } else {
            console.error("invalid response: ", data);
            console.log(PathReporter.report(result));
        }
    });
}
