import { decode_member, decode_string } from "src/decode/decode";

type MutationsFigureProps = {
    kind: "Mutations";
    mutations: MutationVisualProps[];
};

export type MutationVisualProps = { varname: string };

export function decode_mutation(data: unknown): MutationVisualProps {
    let varname = decode_string(decode_member(data, "varname"));
    return { varname };
}

export default MutationsFigureProps;
