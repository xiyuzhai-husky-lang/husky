import type { Writable } from "svelte/store";
import { writable } from "svelte/store";
import type Trace from "../Trace";

export let active_trace_store: Writable<Trace | null> = writable(null);
