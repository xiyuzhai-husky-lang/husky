export function decode_array<T>(
    data: unknown,
    decode_element: (element: unknown) => T
): T[] {
    if (!Array.isArray(data)) {
        throw new Error(`${data} is not a valid array`);
    }
    return data.map((element) => decode_element(element));
}

export function decode_number(data: unknown): number {
    if (typeof data !== "number") {
        throw new Error(`${data} is not a valid number`);
    }
    return data;
}

export function decode_boolean(data: unknown): boolean {
    if (typeof data !== "boolean") {
        throw new Error(`${data} is not a valid boolean`);
    }
    return data;
}

export function decode_number_or_null(data: unknown): number | null {
    return decode_or_null(data, decode_number);
}

export function decode_or_null<T>(
    data: unknown,
    cast: (data: unknown) => T
): T | null {
    if (data === null) {
        return null;
    }
    return cast(data);
}

export function decode_number_or_null_or_undefined(
    data: unknown
): number | null | undefined {
    return decode_or_null_or_undefined(data, decode_number);
}

export function decode_or_null_or_undefined<T>(
    data: unknown,
    cast: (data: unknown) => T
): T | null | undefined {
    if (data === null) {
        return null;
    }
    if (data === undefined) {
        return undefined;
    }
    return cast(data);
}

export function decode_string(data: unknown): string {
    if (typeof data !== "string") {
        throw new Error("this is not a valid string");
    }
    return data;
}

export function decode_member<T>(
    data: unknown,
    key: string,
    cast: (data: unknown) => T
): T {
    if (typeof data !== "object") {
        throw new Error(`expect object but got ${data} instead`);
    }
    if (data === null) {
        throw new Error(`expect object but got ${data} instead`);
    }
    let entries = data as { [key: string]: unknown };
    if (key in entries) {
        return cast(entries[key]);
    } else {
        throw new Error(
            `Key "${key}" doesn't exist in ${JSON.stringify(data)}.`
        );
    }
}
