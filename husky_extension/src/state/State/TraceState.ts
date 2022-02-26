import FutureMap from "src/data_structure/FutureMap";
import SchemeFutureMap from "src/data_structure/SchemeFutureMap";
import type TraceStalk from "src/trace/stalk/TraceStalk";
import type Trace from "src/trace/Trace";
import type { Readable, Writable } from "svelte/store";
import { get, writable } from "svelte/store";
import type RawState from "./InitState";

class TraceState {
    trace_futures: FutureMap<Trace> = new FutureMap();
    subtraces_map: SchemeFutureMap<Trace[]> = new SchemeFutureMap();
    trace_stalk_store_map: SchemeFutureMap<TraceStalk> = new SchemeFutureMap();
    root_traces_store: Writable<Trace[] | null> = writable(null);

    init(raw_state: RawState) {
        console.log("here");
        for (const trace of raw_state.traces) {
            this.trace_futures.set(trace.id, trace);
        }
        let root_traces = [];
        for (const trace_id of raw_state.root_traces) {
            root_traces.push(raw_state.traces[trace_id]);
        }
        this.root_traces_store.set(root_traces);
    }

    get_trace(trace_id: number): Trace {
        let opt_trace = this.trace_futures.get(trace_id);
        if (opt_trace === null) {
            throw new Error("what trace id is this");
        }
        return opt_trace;
    }

    // add_traces(new_traces: Trace[]) {
    //     for (const new_trace of new_traces) {
    //         if (new_trace.id in this.traces) {
    //             console.error("trace already added");
    //         } else {
    //             this.traces[new_trace.id] = new_trace;
    //         }
    //     }
    // }

    receive_subtraces(
        trace_id: number,
        input_id: number | null,
        subtraces: Trace[]
    ) {
        console.log("todo: receive subtraces");
    }

    cache_trace(trace: Trace) {
        console.log("todo");
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

export default TraceState;
