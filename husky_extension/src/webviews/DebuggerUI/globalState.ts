import type { Trace, FigureProps } from "server/types";
import type { Readable, Writable } from "svelte/store";
import { writable, get } from "svelte/store";
import type DebuggerResponse from "./DebuggerResponse";

const version: Writable<number> = writable(0);

const activeTraceIdx_: Writable<number | null> = writable(null);
export const activeTraceIdx: Readable<number | null> = activeTraceIdx_;

let figures: { [idx: number]: Writable<FigureProps | null> } = {};
const figure_: Writable<FigureProps | null> = writable(null);
export const figure: Readable<FigureProps | null> = figure_;

let expansions: { [idx: number]: Writable<boolean> } = {};

let subtraces_table: { [idx: number]: Writable<Trace[] | null> } = {};
let traces: { [idx: number]: Trace } = {};
const rootTraces_: Writable<Trace[]> = writable([]);
export const rootTraces: Readable<Trace[]> = rootTraces_;

let traceList: number[] = [];

const websocket = new WebSocket("ws://localhost:51617/query");
init_websocket(websocket);
function init_websocket(websocket: WebSocket) {
    websocket.addEventListener("open", (event: Event) => {
        websocket.send(rootTracesQuery);
    });
    websocket.addEventListener("message", function (event: MessageEvent) {
        let data: DebuggerResponse = JSON.parse(event.data);

        switch (data.t) {
            case "RootTraces":
                setRootTraces(data.c);
        }
    });

    const rootTracesQuery = JSON.stringify("RootTraces");

    function setRootTraces(traces: Trace[]) {
        rootTraces_.set(traces);
        addTraces(traces);
    }
}

function updateTraceList() {
    for (const trace of get(rootTraces)) {
        updateTraceListDfs(trace.id);
    }

    function updateTraceListDfs(idx: number) {
        traceList.push(idx);
        if (isExpanded(idx)) {
            let subtraces: Trace[] = get(getSubtraces(idx)) || [];
            for (const trace of subtraces) {
                updateTraceListDfs(trace.id);
            }
        }
    }
}

export function toggleExpansion(idx: number) {
    if (hasChildren(idx)) {
        expansions[idx].update((expanded) => !expanded);
        didToggleExpansion(idx);
    }

    function didToggleExpansion(idx: number) {
        updateTraceList();
        console.error("TODO");
    }
}

export function activate(idx: number) {
    if (get(activeTraceIdx) !== idx) {
        activeTraceIdx_.set(idx);
        didActivate(idx);
    }

    function didActivate(idx: number) {
        console.error("TODO");
    }
}

export function onKeyDown(e: KeyboardEvent) {
    switch (e.code) {
        case "KeyH":
            moveLeft();
            break;
        case "KeyL":
            moveRight();
            break;
        case "KeyJ":
            moveDown();
            break;
        case "KeyK":
            moveUp();
            break;
        case "KeyS":
            // console.log(JSON.stringify(globalState));
            break;
        default:
    }

    function moveUp() {
        const idx = get(activeTraceIdx);
        if (idx !== null) {
            const before = idxBefore(idx);
            if (before !== undefined) {
                return activate(before);
            }
        }

        function idxBefore(idx: number): number | undefined {
            return traceList[traceList.indexOf(idx) - 1];
        }
    }

    function moveDown() {
        const idx = get(activeTraceIdx);
        if (idx !== null) {
            const after = idxAfter(idx);
            if (after !== undefined) {
                return activate(after);
            }
        }

        function idxAfter(idx: number): number | undefined {
            return traceList[traceList.indexOf(idx) + 1];
        }
    }

    function moveRight() {
        const idx = get(activeTraceIdx);
        if (idx !== null) {
            if (!isExpanded(idx) && hasChildren(idx)) {
                toggleExpansion(idx);
                moveDown();
            }
        }
    }

    function moveLeft() {
        let idx = get(activeTraceIdx);
        if (idx !== null) {
            const trace = traces[idx];
            if (trace.parent !== null) {
                toggleExpansion(trace.parent);
                activate(trace.parent);
            }
        }
    }
}

function hasChildren(idx: number) {
    const children = get(getSubtraces(idx));
    return children !== null && children.length > 0;
}

export function getDummy() {}

export function isExpanded(id: number): Readable<boolean> {
    if (!(id in expansions)) {
        console.error("id is not in expansions");
    }
    return expansions[id];
}

export function getSubtraces(idx: number | null): Readable<Trace[] | null> {
    if (idx === null) {
        return rootTraces;
    }
    if (idx in subtraces_table) {
        return subtraces_table[idx];
    } else {
        const subtraces = writable(null);
        subtraces_table[idx] = subtraces;
        querySubtraces(idx);
        return subtraces;
    }

    function querySubtraces(idx: number) {
        console.log(get(rootTraces));
        console.error("TODO");
    }
}

function addTraces(new_traces: Trace[]) {
    for (const trace of new_traces) {
        traces[trace.id] = trace;
        expansions[trace.id] = writable(false);
    }
}
