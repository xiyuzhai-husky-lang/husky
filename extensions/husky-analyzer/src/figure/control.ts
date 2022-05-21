import { decode_memb, decode_number_or_null } from "src/decode/decode";
import type { Updater } from "svelte/store";

export class FigureControlProps {
    opt_mutation_selection: number | null;

    constructor(opt_mutation_selection: number | null) {
        this.opt_mutation_selection = opt_mutation_selection;
    }

    select_mutation(mutation_idx: number): FigureControlProps {
        this.opt_mutation_selection = mutation_idx;
        return this;
    }
}

export function decode_figure_control_props(data: unknown): FigureControlProps {
    let opt_mutation_selection = decode_number_or_null(
        decode_memb(data, "opt_mutation_selection")
    );
    return new FigureControlProps(opt_mutation_selection);
}

export function select_mutation(
    mutation_idx: number
): Updater<FigureControlProps> {
    return (control_props: FigureControlProps) => {
        return control_props.select_mutation(mutation_idx);
    };
}
