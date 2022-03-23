import { init_websocket } from "./init_websocket";

export const websocket = new WebSocket("ws://localhost:51617/query");
init_websocket(websocket);
