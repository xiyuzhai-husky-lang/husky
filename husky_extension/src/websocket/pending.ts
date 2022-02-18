export let pending_requests: any[] = [];

export function send_pending_requests(websocket: WebSocket) {
    for (const request of pending_requests) {
        websocket.send(JSON.stringify(request));
    }
    pending_requests = [];
}
