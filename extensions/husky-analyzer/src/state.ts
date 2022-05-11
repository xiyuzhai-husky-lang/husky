import type { DebuggerServer } from "./server";
import server from "./server";
import { TraceState } from "./state/trace";
import { FigureState } from "./state/figure";
import { InitState } from "./state/init";
import { FocusState } from "./state/focus";
import type { Focus } from "./focus";
import type FigureProps from "./figure/FigureProps";
import {
    derived,
    get,
    readable,
    type Readable,
    type Writable,
} from "svelte/store";
import type { Trace } from "./trace";
import type FigureControlProps from "./figure/FigureControlProps";

class DebuggerState {
    private server: DebuggerServer;
    trace_state: TraceState = new TraceState();
    figure_state: FigureState = new FigureState();
    focus_state: FocusState = new FocusState();

    constructor(server: DebuggerServer) {
        this.server = server;
        server.send_request({ kind: "Init" }, (response_variant) => {
            this.init(response_variant);
        });
    }

    init(response_variant: unknown) {
        let init_state = new InitState(response_variant);
        this.trace_state.init(init_state);
        this.figure_state.init(init_state);
        this.focus_state.init(init_state);
    }

    activate(trace_id: number) {
        const focus = get(focus_store);
        throw new Error("todo");
        // server.request_activate(
        //     trace_id,
        //     global.figure_cache.is_figure_cached(trace_id, focus) ? null : focus
        // );
    }

    move_up() {
        let active_trace = get(active_trace_store);
        if (active_trace !== null) {
            const before = this.trace_state.get_id_before(active_trace.id);
            if (before !== undefined) {
                this.activate(before);
            }
        }
    }
    move_down() {
        let active_trace = get(active_trace_store);
        if (active_trace !== null) {
            const after = this.trace_state.get_id_after(active_trace.id);
            if (after !== undefined) {
                return this.activate(after);
            }
        }
    }

    move_right() {
        let active_trace = get(active_trace_store);
        if (active_trace !== null) {
            if (
                !this.trace_state.control.is_expanded(active_trace.id) &&
                active_trace.has_subtraces
            ) {
                this.toggle_expansion(active_trace);
                this.move_down();
            }
        }
    }

    move_left() {
        let active_trace = get(active_trace_store);
        if (active_trace !== null) {
            if (active_trace.parent !== null) {
                this.toggle_expansion(this.get_trace(active_trace.parent));
                this.activate(active_trace.parent);
            }
        }
    }

    get_trace(trace_id: number): Trace {
        return this.trace_state.cache.traces.get(trace_id);
    }

    get_subtraces(focus: Focus, trace_id: number): Trace[] | null {
        let trace = this.trace_state.cache.get_trace(trace_id);
        if (trace === null) {
            return null;
        }
        if (!trace.has_subtraces) {
            return null;
        }
        return this.trace_state.cache.get_subtraces(focus, trace);
    }

    get_expansion_store(trace_id: number) {
        return this.trace_state.control.expansion_stores.get_store_or_insert(
            trace_id,
            false
        );
    }

    get_show_store(trace: Trace) {
        return this.trace_state.control.shown_stores.get_store_or_insert_with(
            trace.id,
            () => {
                let result = tell_shown_default(trace);
                return result;
            }
        );

        function tell_shown_default(trace: Trace): boolean {
            switch (trace.kind) {
                case "Main":
                case "FeatureStmt":
                case "FuncStmt":
                case "ProcStmt":
                case "ProcBranch":
                case "FeatureBranch":
                case "CallHead":
                case "LoopFrame":
                case "FeatureCallInput":
                    return true;
                case "FeatureExpr":
                    return false;
                case "EagerExpr":
                    return trace.parent !== null;
            }
        }
    }

    toggle_expansion(trace: Trace) {
        const expanded = get(this.get_expansion_store(trace.id));
        const request_subtraces = !expanded
            ? !this.trace_state.cache.is_subtraces_cached(
                  get(focus_store),
                  trace
              )
            : false;
        throw new Error("todo");
        // server.request_toggle_expansion(
        //     trace.id,
        //     get(
        //         global.user_state.focus_store
        //     ).gen_subtraces_effective_opt_input_id(trace),
        //     request_subtraces
        // );
    }

    gen_has_subtraces_store(trace: Trace | null): Readable<boolean> {
        if (trace === null) {
            return readable(false);
        }
        switch (trace.kind) {
            case "Main":
            case "FeatureBranch":
            case "LoopFrame":
                return readable(true);
            case "CallHead":
            case "FeatureCallInput":
            case "FeatureStmt":
                return readable(false);
            case "FuncStmt":
            case "EagerExpr":
            case "ProcStmt":
            case "ProcBranch":
                return readable(trace.has_subtraces);
            case "FeatureExpr":
                let focus_store = state.focus_state.focus_store;
                return derived(
                    focus_store,
                    ($focus_store) =>
                        $focus_store.opt_input_id !== null &&
                        trace.has_subtraces
                );
        }
    }

    figure_props(active_trace_id: number, focus: Focus): FigureProps {
        try {
            let figure_props = this.figure_state.get_figure(
                active_trace_id,
                focus
            );
            return figure_props;
        } catch (error) {
            console.log("focus", get(focus_store));
            throw error;
        }
    }

    figure_control_store(trace: Trace): Readable<FigureControlProps> {
        try {
            return this.figure_state.get_figure_control_store(trace);
        } catch (error) {
            console.log("focus", get(focus_store));
            throw error;
        }
    }

    print_state() {
        throw new Error("todo");
    }
}

const state = new DebuggerState(server);

export default state;

export const focus_store = state.focus_state.focus_store;
export const active_trace_store = state.trace_state.control.active_trace_store;
export const input_locked_store: Writable<boolean> =
    state.focus_state.focus_locked_store;
