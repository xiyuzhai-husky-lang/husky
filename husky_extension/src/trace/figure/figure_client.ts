import type FigureProps from "./FigureProps";
import type { Readable } from "svelte/store";
import { get_active_trace_id } from "trace/activate/client";
import { figures, current_figure } from "./figure_cache";

export function update_current_figure() {
    let active_trace_id = get_active_trace_id();
    if (active_trace_id !== null && active_trace_id in figures) {
        current_figure.set(figures[active_trace_id]);
    } else {
        current_figure.set(null);
    }
}
export function get_active_figure_store(): Readable<FigureProps | null> {
    return current_figure;
}
