import * as t from "io-ts";
import {
    decode_array,
    decode_boolean,
    decode_member,
    decode_number,
    decode_number_or_null,
    decode_string,
} from "src/cast/cast";

export const tTokenProps = t.type({
    kind: t.union([
        t.literal("special"),
        t.literal("scope"),
        t.literal("keyword"),
        t.literal("label"),
        t.literal("ident"),
        t.literal("literal"),
        t.literal("fade"),
        t.literal("error"),
    ]),
    value: t.string,
    associated_trace: t.union([t.number, t.null]),
});
export type TokenProps = t.TypeOf<typeof tTokenProps>;

export const tLineProps = t.type({
    indent: t.number,
    tokens: t.array(tTokenProps),
    idx: t.number,
});
export type LineProps = t.TypeOf<typeof tLineProps>;

class Trace {
    id: number;
    parent: number | null;
    lines: LineProps[];
    has_subtraces: boolean;
    kind:
        | "Main"
        | "CallHead"
        | "LazyStmt"
        | "LazyBranch"
        | "LazyExpr"
        | "StrictDeclStmt"
        | "ImprStmt"
        | "LoopFrame"
        | "StrictExpr";
    subtraces_container_class: "Call" | null;
    constructor(props: unknown) {
        this.id = decode_member(props, "id", decode_number);
        this.parent = decode_member(props, "parent", decode_number_or_null);
        this.lines = decode_member(props, "lines", (data) =>
            decode_array(data, (element) => element as LineProps)
        );
        this.has_subtraces = decode_member(
            props,
            "has_subtraces",
            decode_boolean
        );
        const kind = decode_member(props, "kind", decode_string);
        switch (kind) {
            case "Main":
            case "CallHead":
            case "LazyStmt":
            case "LazyBranch":
            case "LazyExpr":
            case "StrictDeclStmt":
            case "ImprStmt":
            case "LoopFrame":
            case "StrictExpr":
                this.kind = kind;
                break;
            default:
                throw new Error("Unknown kind");
        }
        this.subtraces_container_class = decode_member(
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
