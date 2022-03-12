import * as t from "io-ts";

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

export const tTrace = t.type({
    id: t.number,
    parent: t.union([t.number, t.null]),
    indent: t.number,
    tokens: t.array(tTokenProps),
    has_subtraces: t.boolean,
    kind: t.union([
        t.literal("Main"),
        t.literal("CallHead"),
        t.literal("LazyStmt"),
        t.literal("LazyBranch"),
        t.literal("LazyExpr"),
        t.literal("StrictDeclStmt"),
        t.literal("ImprStmt"),
        t.literal("LoopFrame"),
        t.literal("StrictExpr"),
    ]),
    subtraces_container_class: t.union([t.literal("Call"), t.null]),
});
type Trace = t.TypeOf<typeof tTrace>;
export default Trace;
