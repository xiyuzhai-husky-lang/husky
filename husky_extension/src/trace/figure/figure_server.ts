import type FigureProps from "./FigureProps";
import { get_active_trace } from "src/trace/active/active_trace_server";
import { figures, current_figure } from "./figure_cache";

export function has_figure(id: number): boolean {
    return id in figures;
}
export function set_figure(id: number, figure: FigureProps) {
    figures[id] = figure;
    let active_trace = get_active_trace();
    if (active_trace !== null && id === active_trace.id) {
        current_figure.set(figure);
    }
}
