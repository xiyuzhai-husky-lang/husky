import type { Readable, Writable } from "svelte/store";
import { input_store, input_locked_store } from "./input_store";

export function get_input_store(): Readable<number | null> {
    return input_store;
}

export function get_input_locked_store(): Writable<boolean> {
    return input_locked_store;
}
