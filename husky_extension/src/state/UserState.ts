import type Trace from "src/trace/Trace";
import type { Writable } from "svelte/store";
import { writable, get } from "svelte/store";
import type InitState from "./InitState";
import StoreMap from "src/data_structure/StoreMap";

class UserState {
    //volatile
    active_trace_store: Writable<Trace | null> = writable(null);
    expansion_stores: StoreMap<boolean> = new StoreMap();
    shown_stores: StoreMap<boolean> = new StoreMap();
    opt_input_id_store: Writable<number | null> = writable(0);
    input_locked_store: Writable<boolean> = writable(true);

    init(raw_state: InitState) {
        this.active_trace_store.set(
            raw_state.active_trace_id === null
                ? null
                : raw_state.traces[raw_state.active_trace_id]
        );
        this.expansion_stores.load(raw_state.expansions);
        this.shown_stores.load(raw_state.showns);
        this.opt_input_id_store.set(raw_state.opt_input_id);
        console.log("raw_state.opt_input_id", raw_state.opt_input_id);
        this.input_locked_store.set(true);
    }

    is_expanded(trace_id: number): boolean {
        return this.expansion_stores.get_or(trace_id, false);
    }

    is_shown(trace_id: number): boolean {
        return this.shown_stores.get_or(trace_id, false);
    }

    opt_input_id(): number | null {
        return get(this.opt_input_id_store);
    }

    active_trace(): Trace | null {
        return get(this.active_trace_store);
    }

    did_toggle_expansion(trace_id: number) {
        this.expansion_stores.update(trace_id, (expanded) => !expanded);
    }

    did_toggle_show(id: number) {
        this.shown_stores.update(id, (shown) => !shown);
    }

    did_lock_input(
        input_locked_on: number | null | undefined,
        message: string | null
    ) {
        if (input_locked_on !== undefined) {
            this.opt_input_id_store.set(input_locked_on);
            this.input_locked_store.set(true);
        } else {
            alert!(message);
        }
    }
}

export default UserState;

function load_store_table<T>(value_table: { [id_str: string]: T }): {
    [id: number]: Writable<T>;
} {
    let store_table: { [id: number]: Writable<T> } = {};
    for (const id_str in value_table.showns) {
        const id = parseInt(id_str);
        store_table[id] = writable(value_table[id]);
    }
    return store_table;
}
