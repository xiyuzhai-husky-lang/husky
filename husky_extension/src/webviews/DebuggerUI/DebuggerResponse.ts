import { Trace } from "src/server/types";

type DebuggerResponse = RootTracesResponse;
export default DebuggerResponse;
type RootTracesResponse = { t: "RootTraces"; c: Trace[] };
