export default class Dict<T> {
    data: { [key: string]: T } = {};

    clear() {
        this.data = {};
    }

    insert_new(key: string, value: T) {
        console.assert(!(key in this.data));
        this.data[key] = value;
    }

    get(key: string, gen_error_message: () => string) {
        console.assert(key in this.data, gen_error_message());
        return this.data[key];
    }

    has(key: string): boolean {
        return key in this.data;
    }

    entries() {
        return Object.entries(this.data);
    }
}

export function decode_dict<T>(
    raw_data: unknown,
    decode_element: (data: unknown) => T
): Dict<T> {
    if (typeof raw_data !== "object") {
        throw new TypeError("todo");
    }
    if (raw_data === null) {
        throw new TypeError("todo");
    }
    let result = new Dict<T>();
    for (const [key, value] of Object.entries(raw_data)) {
        result.insert_new(key, decode_element(value));
    }
    return result;
}
