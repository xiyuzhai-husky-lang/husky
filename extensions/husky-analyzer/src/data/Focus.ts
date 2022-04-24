import { decode_member, decode_number, decode_opt } from "src/decode/decode";
import type Trace from "src/trace/Trace";

export default class Focus {
    opt_input_id: number | null;

    constructor(data?: unknown) {
        if (data === undefined) {
            this.opt_input_id = null;
        } else {
            this.opt_input_id = decode_opt(
                decode_member(data, "opt_input_id"),
                decode_number
            );
        }
    }

    gen_figure_key(trace_id: number): string {
        return `${this.key()}:${trace_id}`;
    }

    key(): string {
        if (this.opt_input_id === null) {
            return "g";
        } else {
            return JSON.stringify(this.opt_input_id);
        }
    }

    gen_subtraces_key(trace: Trace): string {
        return `${this.gen_subtraces_effective_opt_input_id(trace)}:${
            trace.id
        }`;
    }

    gen_subtraces_effective_opt_input_id(trace: Trace): number | null {
        switch (trace.kind) {
            case "Main":
            case "FeatureStmt":
            case "FuncStmt":
            case "ProcStmt":
            case "LoopFrame":
            case "FeatureBranch":
            case "FeatureCallInput":
                return null;
            case "FeatureExpr":
            case "CallHead":
            case "EagerExpr":
                return this.opt_input_id;
        }
    }
}
