import { get } from "svelte/store";
import { trace_listing_store } from "src/trace/listing/listing_store";

export function get_id_before(id: number): number | undefined {
    let trace_listing = get(trace_listing_store);
    return trace_listing[trace_listing.indexOf(id) - 1];
}

export function get_id_after(id: number): number | undefined {
    let trace_listing = get(trace_listing_store);
    return trace_listing[trace_listing.indexOf(id) - 1];
}
