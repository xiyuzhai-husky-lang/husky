import type FigureProps from "./FigureProps";
import { get_active_trace_id } from "trace/activate/server";
import { figures, current_figure } from "./figure_cache";

export function has_figure(id: number): boolean {
    return id in figures;
}
export function set_figure(id: number, figure: FigureProps) {
    figures[id] = figure;
    if (id === get_active_trace_id()) {
        current_figure.set(figure);
    }
}
