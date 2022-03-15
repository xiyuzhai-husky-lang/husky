import FutureMap from "src/data_structure/FutureMap";
import SchemeFutureMap from "src/data_structure/SchemeFutureMap";
import type TraceStalk from "src/trace/stalk/TraceStalk";
import type Trace from "src/trace/Trace";
import type { Writable } from "svelte/store";
import { writable, get } from "svelte/store";
import type InitState from "./InitState";

class TraceCache {
    trace_futures: FutureMap<Trace> = new FutureMap();
    subtraces_map: SchemeFutureMap<Trace[]> = new SchemeFutureMap();
    trace_stalk_store_map: SchemeFutureMap<TraceStalk> = new SchemeFutureMap();
    root_traces_store: Writable<Trace[] | null> = writable(null);

    init(init_state: InitState) {
        this.init_trace_futures(init_state.traces);
        this.init_root_traces_stores(init_state.traces, init_state.root_traces);
        this.init_subtraces_map(init_state.traces, init_state.subtraces_list);
    }

    init_trace_futures(traces: Trace[]) {
        this.trace_futures.clear();
        for (const trace of traces) {
            this.trace_futures.set(trace.id, trace);
        }
    }

    init_root_traces_stores(traces: Trace[], root_trace_ids: number[]) {
        let root_traces = [];
        for (const trace_id of root_trace_ids) {
            root_traces.push(traces[trace_id]);
        }
        this.root_traces_store.set(root_traces);
    }

    init_subtraces_map(
        traces: Trace[],
        subtrace_ids_list: [[number, number | null], number[]][]
    ) {
        this.subtraces_map.clear();
        for (const [
            [trace_id, effective_opt_input_id_for_subtraces],
            subtrace_ids,
        ] of subtrace_ids_list) {
            let subtraces = [];
            for (const trace_id of subtrace_ids) {
                subtraces.push(traces[trace_id]);
            }
            this.subtraces_map.set(
                trace_id,
                effective_opt_input_id_for_subtraces,
                subtraces
            );
        }
    }

    get_trace(trace_id: number): Trace | null {
        let opt_trace = this.trace_futures.get(trace_id);
        if (opt_trace === null) {
            return null;
        }
        return opt_trace;
    }

    get_subtraces(
        trace_id: number,
        effective_input_id_for_subtraces: number | null
    ): Trace[] | null {
        return this.subtraces_map.get(
            trace_id,
            effective_input_id_for_subtraces
        );
    }

    receive_subtraces(
        trace_id: number,
        input_id: number | null,
        subtraces: Trace[]
    ) {
        this.cache_traces(subtraces);
        this.subtraces_map.set(trace_id, input_id, subtraces);
    }

    cache_traces(traces: Trace[]): void {
        for (const trace of traces) {
            this.cache_trace(trace);
        }
    }

    cache_trace(trace: Trace) {
        this.trace_futures.set(trace.id, trace);
    }

    set_trace_stalk(trace_id: number, input_id: number, stalk: TraceStalk) {
        this.trace_stalk_store_map.set(trace_id, input_id, stalk);
        // this.trace_stalk_stores_table[trace_id][input_id].set(stalk);
    }

    get_trace_stalk_store(
        trace_id: number,
        input_id: number,
        make_request: () => void
    ) {
        return this.trace_stalk_store_map.get_store(
            trace_id,
            input_id,
            make_request
        );
    }
}

export default TraceCache;
