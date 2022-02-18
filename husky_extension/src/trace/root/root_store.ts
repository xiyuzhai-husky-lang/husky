import type { Writable } from "svelte/store";
import { writable } from "svelte/store";
import type Trace from "../Trace";

export let root_traces: Writable<Trace[] | null> = writable(null);
