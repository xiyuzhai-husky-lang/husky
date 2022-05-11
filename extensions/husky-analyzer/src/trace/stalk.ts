import type { TokenProps } from "../../old/Trace";

type TraceStalk = {
    extra_tokens: TokenProps[];
};
export default TraceStalk;

export function decode_trace_stalk(data: unknown): TraceStalk {
    return data as TraceStalk;
}
