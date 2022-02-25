import * as t from "io-ts";

export const tTokenProps = t.interface({
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

export const tTrace = t.interface({
    id: t.number,
    parent: t.union([t.number, t.null]),
    indent: t.number,
    tokens: t.array(tTokenProps),
    has_subtraces: t.boolean,
    kind: t.union([
        t.literal("Main"),
        t.literal("FeatureStmt"),
        t.literal("FeatureBranch"),
        t.literal("FeatureExpr"),
        t.literal("DeclStmt"),
    ]),
    subtraces_container_class: t.union([t.literal("Call"), t.null]),
});
export type Trace = t.TypeOf<typeof tTrace>;
export default Trace;
