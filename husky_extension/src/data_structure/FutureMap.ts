import type { Readable, Writable } from "svelte/store";
import { writable, get } from "svelte/store";

class FutureStore<T> {
    private values: { [id: number]: Writable<T | null> } = {};

    set(id: number, value: T) {
        if (id in this.values) {
            this.values[id].update((old) => {
                if (old !== null) {
                    throw new Error("what");
                }
                return value;
            });
        }
    }

    get_store(
        id: number,
        make_request: () => void = () => {}
    ): Readable<T | null> {
        if (id in this.values) {
            return this.values[id];
        } else {
            make_request();
            return (this.values[id] = writable(null));
        }
    }

    get(id: number, make_request: () => void = () => {}): T | null {
        if (id in this.values) {
            return get(this.values[id]);
        } else {
            make_request();
            return get((this.values[id] = writable(null)));
        }
    }
}

export default FutureStore;
