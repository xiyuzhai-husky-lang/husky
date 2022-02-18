import type FigureProps from "./FigureProps";
import type { Readable } from "svelte/store";
import { get_active_trace } from "src/trace/active/active_trace_client";
import { figures, current_figure } from "./figure_cache";

export function update_current_figure() {
    let active_trace = get_active_trace();
    if (active_trace !== null && active_trace.id in figures) {
        current_figure.set(figures[active_trace.id]);
    } else {
        current_figure.set(null);
    }
}
export function get_active_figure_store(): Readable<FigureProps | null> {
    return current_figure;
}
