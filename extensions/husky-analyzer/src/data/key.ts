import type Trace from "src/trace/Trace";

export function gen_subtraces_key(
    effective_opt_input_id_for_subtraces: number | null,
    trace_id: number
): string {
    return `${effective_opt_input_id_for_subtraces}:${trace_id}`;
}

export function gen_subtraces_effective_opt_input_id(
    opt_input_id: number | null,
    trace: Trace
): number | null {
    throw new Error("TODO");
}
