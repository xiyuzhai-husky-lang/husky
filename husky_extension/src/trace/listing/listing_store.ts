import type { Writable } from "svelte/store";
import { writable } from "svelte/store";

export let trace_listing_store: Writable<number[]> = writable([]);
