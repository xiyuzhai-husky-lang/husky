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
    return decode_opt(data, decode_number);
}

export function decode_opt<T>(
    data: unknown,
    decode_valid: (data: unknown) => T
): T | null {
    if (data === null) {
        return null;
    }
    return decode_valid(data);
}

export function decode_number_or_null_or_undefined(
    data: unknown
): number | null | undefined {
    return decode_opt_or_undefined(data, decode_number);
}

export function decode_opt_or_undefined<T>(
    data: unknown,
    decode_valid: (data: unknown) => T
): T | null | undefined {
    if (data === null) {
        return null;
    }
    if (data === undefined) {
        return undefined;
    }
    return decode_valid(data);
}

export function decode_string(data: unknown): string {
    if (typeof data !== "string") {
        throw new Error(`${data} is not a valid string`);
    }
    return data;
}

export function d_memb_old<T>(
    data: unknown,
    key: string,
    d_memb: (data: unknown) => T
): T {
    if (typeof data !== "object") {
        throw new Error(`expect object but got ${data} instead`);
    }
    if (data === null) {
        throw new Error(`expect object but got ${data} instead`);
    }
    let entries = data as { [key: string]: unknown };
    if (key in entries) {
        try {
            return d_memb(entries[key]);
        } catch (error) {
            console.error(
                `failed to decode member ${key} in ${JSON.stringify(data)}`
            );
            throw error;
        }
    } else {
        throw new Error(
            `Key "${key}" doesn't exist in ${JSON.stringify(data)}.`
        );
    }
}

export function decode_memb(data: unknown, key: string): unknown {
    if (typeof data !== "object") {
        throw new Error(
            `expect object but got type ${typeof data} with data = ${data} instead`
        );
    }
    if (data === null) {
        throw new Error(`expect object but got ${data} instead`);
    }
    let entries = data as { [key: string]: unknown };
    if (key in entries) {
        return entries[key];
    } else {
        throw new Error(
            `Key "${key}" doesn't exist in ${JSON.stringify(data)}.`
        );
    }
}

export function decode_number_pair(data: unknown): [number, number] {
    let array = decode_array(data, decode_number);
    if (array.length !== 2) {
        throw new Error("TODO");
    }
    return array as [number, number];
}
