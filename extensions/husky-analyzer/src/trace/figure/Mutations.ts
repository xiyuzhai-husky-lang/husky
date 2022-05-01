import { d_memb, d_string } from "src/decode/decode";
import type VisualProps from "./VisualProps";
import { d_visual_props } from "./VisualProps";

type MutationsFigureProps = {
    kind: "Mutations";
    mutations: MutationVisualProps[];
};

export type MutationVisualProps = {
    varname: string;
    before: VisualProps;
    after: VisualProps;
};

export function decode_mutation(data: unknown): MutationVisualProps {
    let varname = d_string(d_memb(data, "varname"));
    let before = d_visual_props(d_memb(data, "before"));
    let after = d_visual_props(d_memb(data, "after"));
    return { varname, before, after };
}

export default MutationsFigureProps;
