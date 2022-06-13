import type { Trace } from "src/trace";
import type { Readable, Updater, Writable } from "svelte/store";
import type { FigureControlData } from "src/figure/control";
import StoreStringMap from "src/abstraction/StoreStringMap";
import type { InitState } from "./init_state";
import type FigureContentProps from "src/figure";
import type { Focus } from "src/focus";
import Dict from "src/abstraction/Dict";

export class FigureState {
    figures: Dict<FigureContentProps> = new Dict<FigureContentProps>();
    figure_control_stores: StoreStringMap<FigureControlData> =
        new StoreStringMap();

    init(init_state: InitState) {
        this.figures = init_state.figures;
        this.figure_control_stores.load(init_state.figure_controls);
    }

    set_figure(
        trace: Trace,
        focus: Focus,
        figure: FigureContentProps,
        figure_control_props: FigureControlData
    ) {
        let key = focus.gen_figure_key(trace.id);
        console.assert(!(key in this.figures));
        this.figures.insert_new(key, figure);
        this.set_figure_control_props(trace, figure_control_props);
    }

    get_figure(trace_id: number, focus: Focus): FigureContentProps {
        let key = focus.gen_figure_key(trace_id);
        // if (!(key in this.figures)) {
        //     throw new Error(
        //         `key ${JSON.stringify(key)} not in figures: ${JSON.stringify(
        //             this.figures
        //         )}`
        //     );
        // }
        return this.figures.get(
            key,
            () =>
                `key ${JSON.stringify(key)} not in figures: ${JSON.stringify(
                    this.figures
                )}`
        );
    }

    is_figure_cached(trace_id: number, focus: Focus): boolean {
        let key = focus.gen_figure_key(trace_id);
        return this.figures.has(key);
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
        figure_control_props: FigureControlData
    ) {
        let key = this.figure_control_key(trace);
        this.figure_control_stores.set(key, figure_control_props);
    }

    update_figure_control_props(
        trace: Trace,
        updater: Updater<FigureControlData>
    ) {
        let key = this.figure_control_key(trace);
        this.figure_control_stores.update(key, updater);
    }

    get_figure_control_store(trace: Trace): Readable<FigureControlData> {
        let key = this.figure_control_key(trace);
        return this.figure_control_stores.get_store(key);
    }
}
