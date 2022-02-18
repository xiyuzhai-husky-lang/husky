import type { Readable, Writable } from "svelte/store";
import { get } from "svelte/store";
import { input_id_store, input_locked_store } from "./input_store";

export function get_input(): number | null {
    return get(input_id_store);
}

export function get_input_id_store(): Readable<number | null> {
    return input_id_store;
}

export function get_input_locked_store(): Writable<boolean> {
    return input_locked_store;
}
