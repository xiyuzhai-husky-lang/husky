import type { Readable, Writable } from "svelte/store";
import { input_id_store, input_locked_store } from "./input_store";

export function init_input(opt_input_id: number | null) {
    input_id_store.set(opt_input_id);
    input_locked_store.set(true);
}

export function did_lock_input(
    input_locked_on: number | null | undefined,
    message: string | null
) {
    if (input_locked_on !== undefined) {
        input_id_store.set(input_locked_on);
        input_locked_store.set(true);
    } else {
        alert!(message);
    }
}
