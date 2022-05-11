import NDict from "./abstraction/NDict";
import { decode_memb, decode_number } from "./decode/decode";

type DebuggerResponse = {
    id: number;
    variant: any;
};

type DebuggerRequest = { request_id?: number; variant: unknown };

function decode_debugger_response(raw: unknown): DebuggerResponse {
    let id = decode_number(decode_memb(raw, "request_id"));
    let variant = decode_memb(raw, "variant");
    return { id, variant };
}

export class DebuggerServer {
    websocket = new WebSocket("ws://localhost:51617/query");
    call_backs: NDict<(response_variant: unknown) => void> = new NDict();
    pending_requests: DebuggerRequest[] = [];
    private next_request_id = 0;

    constructor() {
        this.init_websocket();
    }

    private init_websocket() {
        this.websocket.addEventListener("open", (event: Event) => {
            this.send_pending_requests();
        });
        this.websocket.addEventListener(
            "message",
            function (this: DebuggerServer, event: MessageEvent) {
                let response: DebuggerResponse = decode_debugger_response(
                    event.data
                );
                this.call_backs.get(response.id)(response.variant);
            }.bind(this)
        );
    }

    private send_pending_requests(): void {
        for (const request of this.pending_requests) {
            this.websocket.send(JSON.stringify(request));
        }
        this.pending_requests = [];
    }

    private gen_request_id(): number {
        let request_id = this.next_request_id;
        this.next_request_id++;
        return request_id;
    }

    send_request(
        variant: any,
        call_back?: (response: DebuggerResponse) => void
    ) {
        let request: DebuggerRequest = {
            variant,
        };
        if (call_back !== undefined) {
            let request_id = this.gen_request_id();
            request.request_id = request_id;
            this.call_backs.insert_new(request_id, variant);
        }
        switch (this.websocket.readyState) {
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

const server = new DebuggerServer();

export default server;
