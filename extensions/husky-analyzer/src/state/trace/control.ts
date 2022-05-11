import StoreMap from "src/abstraction/StoreMap";
import type { Trace } from "src/trace";
import { writable, type Writable } from "svelte/store";
import type { InitState } from "../init";

export class TraceControlState {
    active_trace_store: Writable<Trace | null> = writable(null);
    expansion_stores: StoreMap<boolean> = new StoreMap();
    shown_stores: StoreMap<boolean> = new StoreMap();

    init(init_state: InitState): void {
        this.active_trace_store.set(
            init_state.active_trace_id === null
                ? null
                : init_state.traces[init_state.active_trace_id]
        );
        this.expansion_stores.load(init_state.expansions);
        this.shown_stores.load(init_state.showns);
    }

    is_expanded(trace_id: number): boolean {
        return this.expansion_stores.get_or(trace_id, false);
    }

    did_toggle_expansion(trace_id: number) {
        this.expansion_stores.update(trace_id, (expanded) => !expanded);
    }

    is_shown(trace_id: number): boolean {
        return this.shown_stores.get_or(trace_id, false);
    }

    did_toggle_show(id: number) {
        this.shown_stores.update(id, (shown) => !shown);
    }

    print_state() {
        console.log("trace control:");
        console.log("    shown_stores:");
        this.shown_stores.print_state(8);
        console.log("    expansion_stores:");
        this.expansion_stores.print_state(8);
    }
}
// function load_store_table<T>(value_table: { [id_str: string]: T }): {
//     [id: number]: Writable<T>;
// } {
//     let store_table: { [id: number]: Writable<T> } = {};
//     for (const id_str in value_table.showns) {
//         const id = parseInt(id_str);
//         store_table[id] = writable(value_table[id]);
//     }
//     return store_table;
// }
