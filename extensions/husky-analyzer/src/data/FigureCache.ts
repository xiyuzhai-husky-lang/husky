import type FigureProps from "src/trace/figure/FigureProps";
import type Focus from "./Focus";
import type InitData from "./InitData";

class FigureCache {
    figures: { [focus: string]: FigureProps } = {};

    init(init_data: InitData) {
        this.figures = init_data.figures;
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
}

export default FigureCache;
