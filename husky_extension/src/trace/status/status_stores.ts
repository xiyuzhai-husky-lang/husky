import type { Writable } from "svelte/store";

export let shown_stores: { [id: number]: Writable<boolean> } = {};
export let expansion_stores: { [id: number]: Writable<boolean> } = {};
