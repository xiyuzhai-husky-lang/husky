import type { Focus } from "src/focus";
import type { Trace } from "src/trace";
import { get, writable, type Writable } from "svelte/store";
import type { InitState } from "./init";
import { TraceCache } from "./trace/cache";
import { TraceControlState } from "./trace/control";

export class TraceState {
    cache: TraceCache = new TraceCache();
    control: TraceControlState = new TraceControlState();
    trace_listing_store: Writable<number[]> = writable([]);

    init(init_state: InitState): void {
        this.cache.init(init_state);
        this.control.init(init_state);
        this.update_trace_listing(init_state.focus);
    }

    active_trace(): Trace | null {
        return get(this.control.active_trace_store);
    }

    get_id_before(trace_id: number): number | undefined {
        let trace_listing = get(this.trace_listing_store);
        let index = trace_listing.indexOf(trace_id);
        if (index === -1) {
            throw new Error("panic");
        }
        return trace_listing[trace_listing.indexOf(trace_id) - 1];
    }

    get_id_after(trace_id: number) {
        let trace_listing = get(this.trace_listing_store);
        return trace_listing[trace_listing.indexOf(trace_id) + 1];
    }

    update_trace_listing(focus: Focus) {
        let root_traces = get(this.cache.root_traces_store);
        if (root_traces === null) {
            this.trace_listing_store.set([]);
        } else {
            let trace_listing: number[] = [];
            for (const trace of root_traces) {
                this.update_trace_listing_dfs(focus, trace, trace_listing);
            }
            this.trace_listing_store.set(trace_listing);
        }
    }

    private update_trace_listing_dfs(
        focus: Focus,
        trace: Trace,
        trace_listing: number[]
    ) {
        trace_listing.push(trace.id);
        this.add_associated_traces(focus, trace.id, trace_listing);
        if (this.control.is_expanded(trace.id)) {
            let subtraces = this.cache.get_subtraces(focus, trace);
            if (subtraces !== null) {
                for (const trace of subtraces) {
                    this.update_trace_listing_dfs(focus, trace, trace_listing);
                }
            }
        }
    }

    add_associated_traces(focus: Focus, id: number, trace_listing: number[]) {
        let trace = this.cache.get_trace(id);
        if (trace !== null) {
            for (const line of trace.lines) {
                let tokens = line.tokens;
                for (const token of tokens) {
                    let associated_trace_id = token.associated_trace;
                    if (associated_trace_id !== null) {
                        if (this.control.is_shown(associated_trace_id)) {
                            let associated_trace =
                                this.cache.get_trace(associated_trace_id);
                            if (associated_trace === null) {
                                throw new Error("panic");
                            }
                            this.update_trace_listing_dfs(
                                focus,
                                associated_trace,
                                trace_listing
                            );
                        }
                    }
                }
            }
        }
    }

    print_state() {
        throw new Error("todo");
        // this.user_state.print_state();
        // this.trace_cache.print_state();
    }
}
