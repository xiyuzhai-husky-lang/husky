import type Trace from "src/trace/Trace";
import type { Readable, Writable } from "svelte/store";
import { writable, get } from "svelte/store";
import StoreMap from "src/abstraction/StoreMap";
import type FigureControlProps from "src/figure/FigureControlProps";
import StoreStringMap from "src/abstraction/StoreStringMap";
import type { InitState } from "./init";
import type FigureProps from "src/figure/FigureProps";
import type { Focus } from "src/focus";

export class FigureState {
    figures: { [focus: string]: FigureProps } = {};
    figure_control_stores: StoreStringMap<FigureControlProps> =
        new StoreStringMap();

    init(init_state: InitState) {
        this.figures = init_state.figures;
        this.figure_control_stores.load(init_state.figure_controls);
    }

    set_figure(trace_id: number, focus: Focus, figure: FigureProps) {
        let key = focus.gen_figure_key(trace_id);
        console.assert(!(key in this.figures));
        this.figures[key] = figure;
    }

    get_figure(trace_id: number, focus: Focus): FigureProps {
        let key = focus.gen_figure_key(trace_id);
        if (!(key in this.figures)) {
            throw new Error(
                `key ${JSON.stringify(key)} not in figures: ${JSON.stringify(
                    this.figures
                )}`
            );
        }
        return this.figures[key];
    }

    is_figure_cached(trace_id: number, focus: Focus): boolean {
        let key = focus.gen_figure_key(trace_id);
        return key in this.figures;
    }

    private figure_control_key(trace: Trace): string {
        switch (trace.kind) {
            case "Main":
            case "CallHead":
            case "FeatureStmt":
            case "FeatureBranch":
            case "FeatureExpr":
            case "FeatureCallInput":
            case "FuncStmt":
            case "ProcStmt":
            case "ProcBranch":
            case "EagerExpr":
                return `${trace.id}`;
            case "LoopFrame":
                return `${trace.parent}`;
        }
    }

    set_figure_control_props(
        trace: Trace,
        figure_control_props: FigureControlProps
    ) {
        let key = this.figure_control_key(trace);
        this.figure_control_stores.set(key, figure_control_props);
    }

    get_figure_control_store(trace: Trace): Readable<FigureControlProps> {
        let key = this.figure_control_key(trace);
        return this.figure_control_stores.get_store(key);
    }
}
