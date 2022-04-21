import type { Readable, Writable } from "svelte/store";
import { writable, get } from "svelte/store";

class StoreMap<T> {
    private stores: { [id: number]: Writable<T> } = {};

    load(values: { [id_str: string]: T }) {
        this.stores = {};
        for (const id_str in values) {
            const id = parseInt(id_str);
            this.stores[id] = writable(values[id]);
        }
    }

    get_or(id: number, v: T): T {
        if (id in this.stores) {
            return get(this.stores[id]);
        } else {
            return v;
        }
    }

    get_or_insert(id: number, v: T): T {
        if (id in this.stores) {
            return get(this.stores[id]);
        } else {
            this.stores[id] = writable(v);
            return v;
        }
    }

    get_store_or_insert(id: number, v: T): Readable<T> {
        if (id in this.stores) {
            return this.stores[id];
        } else {
            return (this.stores[id] = writable(v));
        }
    }

    get_store_or_insert_with(id: number, f: () => T): Readable<T> {
        if (id in this.stores) {
            return this.stores[id];
        } else {
            return (this.stores[id] = writable(f()));
        }
    }

    update(id: number, f: (old: T) => T) {
        if (!(id in this.stores)) {
            throw new Error("Invalid");
        }
        this.stores[id].update(f);
    }

    print_state(indent: number) {
        console.log(" ".repeat(indent), "{");
        for (const [key, value] of Object.entries(this.stores)) {
            console.log(" ".repeat(indent + 4), key, ": ", get(value));
        }
        console.log(" ".repeat(indent), "}");
    }
}

export default StoreMap;
