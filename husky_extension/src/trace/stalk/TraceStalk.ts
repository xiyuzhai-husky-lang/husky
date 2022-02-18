import * as t from "io-ts";
import { tTokenProps } from "../Trace";

export const tTraceStalk = t.interface({
    extra_tokens: t.array(tTokenProps),
});
type TraceStalk = t.TypeOf<typeof tTraceStalk>;
export default TraceStalk;
