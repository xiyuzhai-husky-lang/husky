import type { Writable } from "svelte/store";
import { writable } from "svelte/store";

export let input_store: Writable<number | null> = writable(null);
export let input_locked_store: Writable<boolean> = writable(true);
