import * as t from "io-ts";

import { tFigureProps, tTrace } from "src/server/types";

const tRootTracesResponse = t.interface({
    type: t.literal("RootTraces"),
    root_traces: t.array(tTrace),
});
const tSubtracesResponse = t.interface({
    type: t.literal("Subtraces"),
    id: t.number,
    subtraces: t.array(tTrace),
});
const tFigureResponse = t.interface({
    type: t.literal("Figure"),
    id: t.number,
    figure: t.union([tFigureProps, t.null]),
});
const tDidToggleExpansionResponse = t.interface({
    type: t.literal("DidToggleExpansion"),
    id: t.number,
});
const tDidActivateResponse = t.interface({
    type: t.literal("DidActivate"),
    id: t.number,
});
export const tDebuggerResponse = t.union([
    tRootTracesResponse,
    tSubtracesResponse,
    tFigureResponse,
    tDidActivateResponse,
    tDidToggleExpansionResponse,
]);

type DebuggerResponse = t.TypeOf<typeof tDebuggerResponse>;
export default DebuggerResponse;
