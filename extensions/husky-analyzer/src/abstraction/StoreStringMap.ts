import type { Readable, Writable } from "svelte/store";
import { writable, get } from "svelte/store";

class StoreStringMap<T> {
    private stores: { [key: string]: Writable<T> } = {};

    load(values: { [key: string]: T }) {
        this.stores = {};
        for (const key in values) {
            this.stores[key] = writable(values[key]);
        }
    }

    get_store(key: string): Readable<T> {
        if (key in this.stores) {
            return this.stores[key];
        } else {
            // let key = this.figure_control_key(trace);
            // if (!(key in this.figure_control_stores)) {
            //     const entries = Object.entries(this.figure_control_stores);
            //     console.log("entries.length", entries.length);
            //     for (const [key, value] of entries) {
            //         console.log("key: ", key, " value: ", get(value));
            //     }
            //     throw new Error("what");
            // }
            throw new Error("what");
        }
    }

    get_or(key: string, v: T): T {
        if (key in this.stores) {
            return get(this.stores[key]);
        } else {
            return v;
        }
    }

    get_or_insert(key: string, v: T): T {
        if (key in this.stores) {
            return get(this.stores[key]);
        } else {
            this.stores[key] = writable(v);
            return v;
        }
    }

    get_store_or_insert(key: string, v: T): Readable<T> {
        if (key in this.stores) {
            return this.stores[key];
        } else {
            return (this.stores[key] = writable(v));
        }
    }

    get_store_or_insert_with(key: string, f: () => T): Readable<T> {
        if (key in this.stores) {
            return this.stores[key];
        } else {
            return (this.stores[key] = writable(f()));
        }
    }

    update(key: string, f: (old: T) => T) {
        if (!(key in this.stores)) {
            throw new Error("Invalid");
        }
        this.stores[key].update(f);
    }

    print_state(indent: number) {
        console.log(" ".repeat(indent), "{");
        for (const [key, value] of Object.entries(this.stores)) {
            console.log(" ".repeat(indent + 4), key, ": ", get(value));
        }
        console.log(" ".repeat(indent), "}");
    }

    set(key: string, value: T) {
        if (key in this.stores) {
            this.stores[key].set(value);
        } else {
            this.stores[key] = writable(value);
        }
    }
}

export default StoreStringMap;
