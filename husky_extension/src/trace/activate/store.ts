import type { Writable } from "svelte/store";
import { writable } from "svelte/store";

export let active_trace_id_store: Writable<number | null> = writable(0);
