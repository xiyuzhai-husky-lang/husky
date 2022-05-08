import { decode_memb, decode_number_or_null } from "src/decode/decode";

export default class FigureControlProps {
    opt_mutation_selection: number | null;

    constructor(opt_mutation_selection: number | null) {
        this.opt_mutation_selection = opt_mutation_selection;
    }
}

export function decode_figure_control_props(data: unknown): FigureControlProps {
    let opt_mutation_selection = decode_number_or_null(
        decode_memb(data, "opt_mutation_selection")
    );
    return new FigureControlProps(opt_mutation_selection);
}
