import type FigureProps from "src/trace/figure/FigureProps";
import type Trace from "src/trace/Trace";
import type { Writable } from "svelte/store";
import { writable } from "svelte/store";
import type InitState from "./InitState";

class FigureState {
    figures: { [id: number]: FigureProps | null } = {};
    current_figure: Writable<FigureProps | null> = writable(null);

    init(init_state: InitState) {}

    set_figure(
        trace_id: number,
        figure: FigureProps,
        active_trace: Trace | null
    ) {
        this.figures[trace_id] = figure;
        if (active_trace !== null && trace_id === active_trace.id) {
            this.current_figure.set(figure);
        }
    }

    is_figure_cached(id: number): boolean {
        return id in this.figures;
    }
}

export default FigureState;
