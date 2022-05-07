import {
    decode_array,
    decode_boolean,
    d_memb_old,
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
    reachable: boolean;
    kind:
        | "Main"
        | "CallHead"
        | "FeatureStmt"
        | "FeatureBranch"
        | "FeatureExpr"
        | "FeatureCallInput"
        | "FuncStmt"
        | "ProcStmt"
        | "LoopFrame"
        | "EagerExpr";
    subtraces_container_class: "Call" | null;
    constructor(props: unknown) {
        this.id = d_memb_old(props, "id", decode_number);
        this.parent = d_memb_old(props, "parent", decode_number_or_null);
        this.lines = d_memb_old(props, "lines", (data) =>
            decode_array(data, (element) => element as LineProps)
        );
        this.has_subtraces = d_memb_old(props, "has_subtraces", decode_boolean);
        this.reachable = d_memb_old(props, "reachable", decode_boolean);
        const kind = d_memb_old(props, "kind", decode_string);
        switch (kind) {
            case "Main":
            case "CallHead":
            case "FeatureStmt":
            case "FeatureBranch":
            case "FeatureExpr":
            case "FeatureCallInput":
            case "FuncStmt":
            case "ProcStmt":
            case "LoopFrame":
            case "EagerExpr":
                this.kind = kind;
                break;
            default:
                throw new Error(`Unknown kind ${kind}`);
        }
        this.subtraces_container_class = d_memb_old(
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
