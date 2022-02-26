import type { Readable, Writable } from "svelte/store";
import { writable } from "svelte/store";

class SchemeFutureMap<T> {
    private generic_points: { [trace_id: number]: Writable<T | null> } = {};
    private closed_points: {
        [trace_id: number]: { [input_id: number]: Writable<T | null> };
    } = {};

    set(trace_id: number, input_id: number | null, value: T) {
        if (input_id === null) {
            if (trace_id in this.generic_points) {
                this.generic_points[trace_id].set(value);
            } else {
                this.generic_points[trace_id] = writable(value);
            }
        } else {
            let values: { [input_id: number]: Writable<T | null> };
            if (trace_id in this.closed_points) {
                values = this.closed_points[trace_id];
            } else {
                values = this.closed_points[trace_id] = {};
            }
            if (input_id in values) {
                values[input_id].set(value);
            } else {
                values[input_id] = writable(value);
            }
        }
    }

    get_store(
        trace_id: number,
        input_id: number | null,
        make_request: () => void = () => {}
    ): Readable<T | null> {
        if (input_id === null) {
            if (trace_id in this.generic_points) {
                return this.generic_points[trace_id];
            } else {
                make_request();
                return (this.generic_points[trace_id] = writable(null));
            }
        } else {
            let values: { [input_id: number]: Writable<T | null> };
            if (trace_id in this.closed_points) {
                values = this.closed_points[trace_id];
            } else {
                values = this.closed_points[trace_id] = {};
            }
            if (input_id in values) {
                return values[input_id];
            } else {
                make_request();
                return (values[input_id] = writable(null));
            }
        }
    }
}
export default SchemeFutureMap;
