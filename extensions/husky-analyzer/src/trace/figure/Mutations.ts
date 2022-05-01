import { decode_memb, decode_string } from "src/decode/decode";
import type VisualProps from "./VisualProps/VisualProps";
import { decode_visual_props as decode_visual_props } from "./VisualProps/VisualProps";

type MutationsFigureProps = {
    kind: "Mutations";
    mutations: MutationVisualProps[];
};

export type MutationVisualProps = {
    name: string;
    before: VisualProps;
    after: VisualProps;
};

export function decode_mutation(data: unknown): MutationVisualProps {
    let name = decode_string(decode_memb(data, "name"));
    let before = decode_visual_props(decode_memb(data, "before"));
    let after = decode_visual_props(decode_memb(data, "after"));
    return { name, before, after };
}

export default MutationsFigureProps;
