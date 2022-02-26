import type RawState from "./State/InitState";
import TraceCache from "./State/TraceCache";
import FigureState from "./State/FigureState";
import UserState from "./State/UserState";
import type { Writable } from "svelte/store";
import { writable, get } from "svelte/store";
import type Trace from "src/trace/Trace";

class State {
    trace_cache: TraceCache = new TraceCache();
    figure_state: FigureState = new FigureState();
    user_state = new UserState();
    trace_listing_store: Writable<number[]> = writable([]);

    init(raw_state: RawState) {
        this.trace_cache.init(raw_state);
        this.figure_state.init(raw_state);
        this.user_state.init(raw_state);
    }

    update_trace_listing() {
        let root_traces = get(this.trace_cache.root_traces_store);
        if (root_traces === null) {
            this.trace_listing_store.set([]);
        } else {
            let trace_listing: number[] = [];
            for (const trace of root_traces) {
                this.update_trace_listing_dfs(trace, trace_listing);
            }
            this.trace_listing_store.set(trace_listing);
        }
    }

    get_effective_input_id_for_subtraces(trace: Trace): number | null {
        switch (trace.kind) {
            case "Main":
            case "FeatureStmt":
            case "DeclStmt":
            case "FeatureBranch":
                return null;
            case "FeatureExpr":
                return this.user_state.opt_input_id();
        }
    }

    update_trace_listing_dfs(trace: Trace, trace_listing: number[]) {
        this.add_associated_traces(trace.id, trace_listing);
        if (this.user_state.is_expanded(trace.id)) {
            let subtraces = this.trace_cache.get_subtraces(
                trace.id,
                this.get_effective_input_id_for_subtraces(trace)
            );
            if (subtraces !== null) {
                for (const trace of subtraces) {
                    this.update_trace_listing_dfs(trace, trace_listing);
                }
            }
        }
    }

    add_associated_traces(id: number, trace_listing: number[]) {
        let trace = this.trace_cache.get_trace(id);
        if (trace !== null) {
            let tokens = trace.tokens;
            for (const token of tokens) {
                let associated_trace_id = token.associated_trace;
                if (associated_trace_id !== null) {
                    if (this.user_state.is_shown(associated_trace_id)) {
                        trace_listing.push(associated_trace_id);
                    }
                }
            }
        }
    }
}
export default State;

// export function get_id_before(id: number): number | undefined {
//     let trace_listing = get(trace_listing_store);
//     return trace_listing[trace_listing.indexOf(id) - 1];
// }

// export function get_id_after(id: number): number | undefined {
//     let trace_listing = get(trace_listing_store);
//     return trace_listing[trace_listing.indexOf(id) - 1];
// }
