import { get_root_traces } from "src/trace/root/root_server";
import { is_expanded, is_shown } from "src/trace/status/status_server";
import { get_subtraces } from "trace/subtraces/server";
import type Trace from "trace/Trace";
import { get_trace } from "src/trace/trace_server";
import { trace_listing_store } from "src/trace/listing/listing_store";

export function update_trace_listing() {
    let root_traces = get_root_traces();
    if (root_traces === null) {
        trace_listing_store.set([]);
    } else {
        let trace_listing: number[] = [];
        for (const trace of root_traces) {
            update_trace_listing_dfs(trace.id, trace_listing);
        }
        trace_listing_store.set(trace_listing);
    }

    function update_trace_listing_dfs(id: number, trace_listing: number[]) {
        trace_listing_store.update((listing) => {
            listing.push(id);
            return listing;
        });
        add_associated_traces(id, trace_listing);
        if (is_expanded(id)) {
            let subtraces: Trace[] | null = get_subtraces(id);
            if (subtraces !== null) {
                for (const trace of subtraces) {
                    update_trace_listing_dfs(trace.id, trace_listing);
                }
            }
        }

        function add_associated_traces(id: number, trace_listing: number[]) {
            let trace = get_trace(id);
            if (trace !== null) {
                let tokens = trace.tokens;
                for (const token of tokens) {
                    let associated_trace_id = token.associated_trace;
                    if (associated_trace_id !== null) {
                        if (is_shown(associated_trace_id)) {
                            trace_listing.push(associated_trace_id);
                        }
                    }
                }
            }
        }
    }
}
