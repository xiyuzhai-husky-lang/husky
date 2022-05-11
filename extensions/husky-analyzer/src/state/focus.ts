import { Focus } from "src/focus";
import { get, writable, type Writable } from "svelte/store";
import type { InitState } from "./init";

export class FocusState {
    focus_store: Writable<Focus> = writable(new Focus());
    focus_locked_store: Writable<boolean> = writable(true);
    init(init_state: InitState): void {
        this.focus_locked_store.set(true);
        this.focus_store.set(init_state.focus);
    }

    opt_input_id(): number | null {
        return get(this.focus_store).opt_input_id;
    }

    did_lock_focus(focus: Focus) {
        this.focus_store.set(focus);
        this.focus_locked_store.set(true);
    }

    focus(): Focus {
        return get(this.focus_store);
    }
}
