import { DebuggerServer } from "./server";
import { TraceState } from "./state/trace_state";
import { FigureState } from "./state/figure_state";
import { InitState } from "./state/init_state";
import { FocusState } from "./state/focus_state";
import type { Focus } from "./focus";
import type FigureProps from "./figure";
import {
    derived,
    get,
    readable,
    type Readable,
    type Updater,
    type Writable,
} from "svelte/store";
import type { Trace } from "./trace";
import type { FigureControlProps } from "./figure/control";
import { decode_trace_stalk } from "./trace/stalk";
import {
    decode_array,
    decode_memb,
    decode_number_or_null,
} from "./decode/decode";
import { decode_figure_control_props } from "./figure/control";
import { decode_figure_props } from "./figure";
import { decode_trace } from "./trace";

class DebuggerState {
    private server: DebuggerServer = new DebuggerServer();
    trace_state: TraceState = new TraceState();
    figure_state: FigureState = new FigureState();
    focus_state: FocusState = new FocusState();

    constructor() {
        this.server.send_request({ kind: "Init" }, (response_variant) => {
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
        const is_figure_cached = this.figure_state.is_figure_cached(
            trace_id,
            focus
        );
        const trace = this.get_trace(trace_id);
        if (is_figure_cached) {
            this.did_activate(trace);
            this.server.send_request({
                kind: "Activate",
                trace_id,
                opt_focus_for_figure: null,
            });
        } else {
            this.server.send_request(
                {
                    kind: "Activate",
                    trace_id,
                    opt_focus_for_figure: focus,
                },
                (response_variant: unknown) => {
                    let figure_props = decode_figure_props(
                        decode_memb(response_variant, "figure_props")
                    );
                    let figure_control_props = decode_figure_control_props(
                        decode_memb(response_variant, "figure_control_props")
                    );
                    this.did_activate(
                        trace,
                        focus,
                        figure_props,
                        figure_control_props
                    );
                }
            );
        }
    }

    did_activate(
        trace: Trace,
        opt_focus_for_figure: Focus | null = null,
        opt_figure_props: FigureProps | null = null,
        opt_figure_control: FigureControlProps | null = null
    ) {
        if (opt_figure_props !== null) {
            if (opt_focus_for_figure === null) {
                throw new Error("panic");
            }
            if (opt_figure_control === null) {
                throw new Error("panic");
            }
            this.figure_state.set_figure(
                trace,
                opt_focus_for_figure,
                opt_figure_props,
                opt_figure_control
            );
        }
        this.trace_state.control.active_trace_store.set(trace);
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
        const focus = get(focus_store);
        const request_subtraces = !expanded
            ? !this.trace_state.cache.is_subtraces_cached(focus, trace)
            : false;
        if (request_subtraces) {
            this.server.send_request(
                {
                    kind: "ToggleExpansion",
                    trace_id: trace.id,
                    effective_opt_input_id:
                        focus.gen_subtraces_effective_opt_input_id(trace),
                    request_subtraces,
                },
                (response_variant: unknown) => {
                    this.trace_state.cache.receive_subtraces(
                        trace.id,
                        decode_number_or_null(
                            decode_memb(
                                response_variant,
                                "effective_opt_input_id"
                            )
                        ),
                        decode_array(
                            decode_memb(response_variant, "subtraces"),
                            decode_trace
                        )
                    );
                    this.trace_state.cache.cache_traces(
                        decode_array(
                            decode_memb(response_variant, "associated_traces"),
                            decode_trace
                        )
                    );
                    this.trace_state.did_toggle_expansion(focus, trace.id);
                }
            );
        } else {
            this.trace_state.did_toggle_expansion(focus, trace.id);
            this.server.send_request({
                kind: "ToggleExpansion",
                trace_id: trace.id,
                effective_opt_input_id:
                    focus.gen_subtraces_effective_opt_input_id(trace),
                request_subtraces,
            });
        }
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

    update_figure_control_props(updater: Updater<FigureControlProps>) {
        const active_trace = get(active_trace_store);
        if (active_trace === null) {
            throw new Error("todo");
        }
        this.figure_state.update_figure_control_props(
            active_trace,
            (figure_control_props: FigureControlProps) => {
                const new_value = updater(figure_control_props);
                this.server.send_request({
                    kind: "UpdateFigureControlProps",
                    focus: get(focus_store),
                    trace_id: active_trace.id,
                    figure_control_props: new_value,
                });
                return new_value;
            }
        );
    }

    get_trace_stalk_store(trace_id: number, input_id: number) {
        return this.trace_state.cache.get_trace_stalk_store(
            trace_id,
            input_id,
            () => {
                this.server.send_request(
                    {
                        kind: "TraceStalk",
                        trace_id,
                        input_id,
                    },
                    (response_variant: unknown) => {
                        this.trace_state.cache.set_trace_stalk(
                            trace_id,
                            input_id,
                            decode_trace_stalk(
                                decode_memb(response_variant, "stalk")
                            )
                        );
                    }
                );
            }
        );
    }

    toggle_show(trace_id: number) {
        const focus = get(focus_store);
        this.trace_state.did_toggle_show(focus, trace_id);
        this.server.send_request({
            kind: "ToggleShow",
            trace_id,
        });
    }

    print_state() {
        throw new Error("todo");
    }
}

const state = new DebuggerState();

export default state;

export const focus_store = state.focus_state.focus_store;
export const active_trace_store = state.trace_state.control.active_trace_store;
export const input_locked_store: Writable<boolean> =
    state.focus_state.focus_locked_store;
export const root_traces_store = state.trace_state.cache.root_traces_store;
