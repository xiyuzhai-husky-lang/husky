import type Trace from "trace/Trace";

export function shown_default(trace: Trace | null): boolean {
    if (trace === null) {
        return false;
    }
    switch (trace.kind) {
        case "Main":
        case "FeatureStmt":
        case "FeatureBranch":
        case "DeclStmt":
            return true;
        case "FeatureExpr":
            return false;
    }
}

export function expanded_default(id: number): boolean {
    return false;
}
