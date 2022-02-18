import type Trace from "trace/Trace";
import type { Readable } from "svelte/store";
import { get } from "svelte/store";
import { request_root_traces } from "src/websocket/websocket_client";
import { root_traces } from "./root_store";

export function get_root_traces_store(): Readable<Trace[] | null> {
    if (get(root_traces) === null) {
        request_root_traces();
    }
    return root_traces;
}

export function get_root_traces(): Trace[] | null {
    return get(root_traces);
}
