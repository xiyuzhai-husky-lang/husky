import {
    decode_array,
    decode_boolean,
    decode_member_old,
    decode_number,
    decode_number_or_null,
    decode_string,
} from "src/decode/decode";

export type TokenProps = {
    kind:
        | "special"
        | "scope"
        | "keyword"
        | "label"
        | "ident"
        | "literal"
        | "fade"
        | "error";
    value: string;
    associated_trace: number | null;
};

export type LineProps = {
    indent: number;
    tokens: TokenProps[];
    idx: number;
};

class Trace {
    id: number;
    parent: number | null;
    lines: LineProps[];
    has_subtraces: boolean;
    kind:
        | "Main"
        | "CallHead"
        | "FeatureStmt"
        | "FeatureBranch"
        | "FeatureExpr"
        | "StrictDeclStmt"
        | "ImprStmt"
        | "LoopFrame"
        | "StrictExpr";
    subtraces_container_class: "Call" | null;
    constructor(props: unknown) {
        this.id = decode_member_old(props, "id", decode_number);
        this.parent = decode_member_old(props, "parent", decode_number_or_null);
        this.lines = decode_member_old(props, "lines", (data) =>
            decode_array(data, (element) => element as LineProps)
        );
        this.has_subtraces = decode_member_old(
            props,
            "has_subtraces",
            decode_boolean
        );
        const kind = decode_member_old(props, "kind", decode_string);
        switch (kind) {
            case "Main":
            case "CallHead":
            case "FeatureStmt":
            case "FeatureBranch":
            case "FeatureExpr":
            case "StrictDeclStmt":
            case "ImprStmt":
            case "LoopFrame":
            case "StrictExpr":
                this.kind = kind;
                break;
            default:
                throw new Error(`Unknown kind ${kind}`);
        }
        this.subtraces_container_class = decode_member_old(
            props,
            "subtraces_container_class",
            (data) => data as any
        );
    }
}

export function decode_trace(data: unknown): Trace {
    return new Trace(data);
}
export default Trace;
