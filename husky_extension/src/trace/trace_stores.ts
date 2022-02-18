import type { Writable } from "svelte/store";
import type Trace from "./Trace";

export let trace_stores: { [id: number]: Writable<Trace | null> } = {};
