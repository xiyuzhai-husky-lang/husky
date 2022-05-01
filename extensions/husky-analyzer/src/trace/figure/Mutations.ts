import { decode_memb, decode_string } from "src/decode/decode";
import type VisualProps from "./VisualProps/VisualProps";
import { decode_visual_props as decode_visual_props } from "./VisualProps/VisualProps";

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
    let varname = decode_string(decode_memb(data, "varname"));
    let before = decode_visual_props(decode_memb(data, "before"));
    let after = decode_visual_props(decode_memb(data, "after"));
    return { varname, before, after };
}

export default MutationsFigureProps;
