import type DebuggerResponse from "./DebuggerResponse";
import { tDebuggerResponse, tInitResponse } from "./DebuggerResponse";
import { isRight } from "fp-ts/Either";
import reporter from "io-ts-reporters";
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
            console.log("raw data: ", event.data);
            console.error("invalid response: ", data);
            console.log("report: ", reporter.report(result));
            let init_result = tInitResponse.decode(data);
            console.log("init_report: ", reporter.report(init_result));
        }
    });
}
