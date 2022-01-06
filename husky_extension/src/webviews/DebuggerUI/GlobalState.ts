import server from "server/server";
import type { Trace, FigureProps } from "server/types";

class GlobalState {
    readonly version: number;
    readonly activeTraceIdx: number | null;
    private readonly trace_list: number[];
    private readonly expansions: readonly boolean[];

    private formTraceListDfs(
        expansions: readonly boolean[],
        idx: number,
        list: number[]
    ) {
        list.push(idx);
        if (expansions[idx]) {
            for (const child_idx of this.getChildren(idx)) {
                this.formTraceListDfs(expansions, child_idx, list);
            }
        }
    }

    private formTraceList(expansions: readonly boolean[]): number[] {
        let list: number[] = [];
        this.formTraceListDfs(expansions, 0, list);
        return list;
    }

    private idxBefore(idx: number): number | undefined {
        return this.trace_list[this.trace_list.indexOf(idx) - 1];
    }

    private idxAfter(idx: number): number | undefined {
        return this.trace_list[this.trace_list.indexOf(idx) + 1];
    }

    constructor(
        version: number,
        activeTraceIdx: number | null,
        expansions: readonly boolean[]
    ) {
        this.version = version;
        this.activeTraceIdx = activeTraceIdx;
        this.expansions = expansions;
        this.formTraceList = this.formTraceList.bind(this);
        this.formTraceListDfs = this.formTraceListDfs.bind(this);
        this.trace_list = this.formTraceList(expansions);
    }

    getTrace(idx: number): Trace {
        return server.getTrace(this.version, idx);
    }

    isExpanded(idx: number): boolean {
        return this.expansions[idx];
    }

    getChildren(idx: number): number[] {
        return server.getChildren(this.version, idx);
    }

    hasChildren(idx: number): boolean {
        return this.getChildren(idx).length > 0;
    }

    getFigure(): FigureProps {
        if (this.activeTraceIdx !== null) {
            return server.getFigure(this.version, this.activeTraceIdx);
        } else {
            return null;
        }
    }

    activate(idx: number): GlobalState {
        if (this.activeTraceIdx !== idx) {
            let newGlobalState: GlobalState = new GlobalState(
                this.version,
                idx,
                this.expansions
            );
            server.didActivate(this.version, idx);
            return newGlobalState;
        } else {
            return this;
        }
    }

    toggleExpansion(idx: number): GlobalState {
        if (this.hasChildren(idx)) {
            let expansions = [...this.expansions];
            expansions[idx] = !expansions[idx];
            let newGlobalState: GlobalState = new GlobalState(
                this.version,
                idx,
                expansions
            );
            server.didToggleExpansion(this.version, idx);
            return newGlobalState;
        } else {
            return this;
        }
    }

    moveUp(): GlobalState {
        let idx = this.activeTraceIdx;
        if (idx !== null) {
            let before = this.idxBefore(idx);
            if (before !== undefined) {
                return this.activate(before);
            }
        }
        return this;
    }

    moveDown(): GlobalState {
        let idx = this.activeTraceIdx;
        if (idx !== null) {
            let after = this.idxAfter(idx);
            if (after !== undefined) {
                return this.activate(after);
            }
        }
        return this;
    }

    moveRight(): GlobalState {
        let idx = this.activeTraceIdx;
        if (idx !== null) {
            if (!this.isExpanded(idx) && this.hasChildren(idx)) {
                const state0 = this.toggleExpansion(idx);
                const state = state0.moveDown();
                return state;
            }
        }
        return this;
    }

    moveLeft(): GlobalState {
        let idx = this.activeTraceIdx;
        if (idx !== null) {
            let trace = this.getTrace(idx);
            if (trace.parent !== null) {
                return this.toggleExpansion(trace.parent).activate(
                    trace.parent
                );
            }
        }
        return this;
    }
}
export function new_globalState() {
    let version = 0;
    return new GlobalState(
        version,
        server.getActiveTrace(version),
        server.getExpansions(version)
    );
}
export default GlobalState;
