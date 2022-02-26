import type RawState from "./State/InitState";
import TraceState from "./State/TraceState";
import FigureState from "./State/FigureState";
import UserState from "./State/UserState";
import type { Writable } from "svelte/store";
import { writable } from "svelte/store";

class State {
    trace_state: TraceState = new TraceState();
    figure_state: FigureState = new FigureState();
    user_state = new UserState();
    trace_listing_store: Writable<number[]> = writable([]);

    init(raw_state: RawState) {
        this.trace_state.init(raw_state);
        this.figure_state.init(raw_state);
        this.user_state.init(raw_state);
    }

    update_trace_listing() {
        console.log("todo");
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

// export function update_trace_listing() {
//     let root_traces = get_root_traces();
//     if (root_traces === null) {
//         trace_listing_store.set([]);
//     } else {
//         let trace_listing: number[] = [];
//         for (const trace of root_traces) {
//             update_trace_listing_dfs(trace.id, trace_listing);
//         }
//         trace_listing_store.set(trace_listing);
//     }

//     function update_trace_listing_dfs(id: number, trace_listing: number[]) {
//         trace_listing_store.update((listing) => {
//             listing.push(id);
//             return listing;
//         });
//         add_associated_traces(id, trace_listing);
//         if (is_expanded(id)) {
//             let subtraces: Trace[] | null = get_subtraces(id);
//             if (subtraces !== null) {
//                 for (const trace of subtraces) {
//                     update_trace_listing_dfs(trace.id, trace_listing);
//                 }
//             }
//         }

//         function add_associated_traces(id: number, trace_listing: number[]) {
//             let trace = get_trace(id);
//             if (trace !== null) {
//                 let tokens = trace.tokens;
//                 for (const token of tokens) {
//                     let associated_trace_id = token.associated_trace;
//                     if (associated_trace_id !== null) {
//                         if (is_shown(associated_trace_id)) {
//                             trace_listing.push(associated_trace_id);
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
