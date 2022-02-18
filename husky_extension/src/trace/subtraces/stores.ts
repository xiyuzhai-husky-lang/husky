import type Trace from "../Trace";
import type { Writable } from "svelte/store";

export let subtraces_stores: { [id: number]: Writable<Trace[] | null> } = {};
