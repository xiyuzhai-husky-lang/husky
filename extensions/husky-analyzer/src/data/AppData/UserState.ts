import type Trace from "src/trace/Trace";
import type { Writable } from "svelte/store";
import { writable, get } from "svelte/store";
import type InitData from "./InitData";
import StoreMap from "src/abstraction/StoreMap";
import Focus from "../Focus";
import type FigureControlProps from "src/trace/figure/FigureControlProps";

class UserState {
    //volatile
    active_trace_store: Writable<Trace | null> = writable(null);
    expansion_stores: StoreMap<boolean> = new StoreMap();
    shown_stores: StoreMap<boolean> = new StoreMap();
    focus_store: Writable<Focus> = writable(new Focus());
    focus_locked_store: Writable<boolean> = writable(true);
    figure_control_stores: {
        [figure_control_key: string]: FigureControlProps;
    } = {};

    init(init_state: InitData) {
        this.active_trace_store.set(
            init_state.active_trace_id === null
                ? null
                : init_state.traces[init_state.active_trace_id]
        );
        this.expansion_stores.load(init_state.expansions);
        this.shown_stores.load(init_state.showns);
        this.focus_locked_store.set(true);
        this.focus_store.set(init_state.focus);
    }

    is_expanded(trace_id: number): boolean {
        return this.expansion_stores.get_or(trace_id, false);
    }

    is_shown(trace_id: number): boolean {
        return this.shown_stores.get_or(trace_id, false);
    }

    opt_input_id(): number | null {
        return get(this.focus_store).opt_input_id;
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

    did_lock_focus(focus: Focus) {
        this.focus_store.set(focus);
        this.focus_locked_store.set(true);
    }

    private figure_control_key(trace: Trace): string {
        throw new Error("todo");
    }

    set_figure_control() {}

    get_figure_control_store(trace: Trace): FigureControlProps {
        throw new Error("todo");
    }

    print_state() {
        console.log("user state:");
        console.log("    shown_stores:");
        this.shown_stores.print_state(8);
        console.log("    expansion_stores:");
        this.expansion_stores.print_state(8);
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
