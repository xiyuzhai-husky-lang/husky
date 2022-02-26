import type DebuggerMessage from "./DebuggerMessage";
import { tDebuggerMessage, tInitMessage } from "./DebuggerMessage";
import { isRight } from "fp-ts/Either";
import reporter from "io-ts-reporters";
import { send_pending_requests } from "./pending";
import { handle_message } from "./handle_message";

export function init_websocket(websocket: WebSocket) {
    websocket.addEventListener("open", (event: Event) => {
        send_pending_requests(websocket);
    });
    websocket.addEventListener("message", function (event: MessageEvent) {
        let data: DebuggerMessage = JSON.parse(event.data);
        const result = tDebuggerMessage.decode(data);
        if (isRight(result)) {
            handle_message(data);
        } else {
            console.log("raw data: ", event.data);
            console.error("invalid response: ", data);
            console.log("report: ", reporter.report(result));
            let init_result = tInitMessage.decode(data);
            console.log("init_report: ", reporter.report(init_result));
        }
    });
}
