import { d_string } from "src/decode/decode";

type Result<T> = { kind: "Ok"; value: T } | { kind: "Err"; message: String };
export default Result;

export function decode_result<T>(
    data: unknown,
    decode_value: (raw: unknown) => T
): Result<T> {
    if (typeof data !== "object") {
        throw new Error(`expect object but got ${data} instead`);
    }
    if (data === null) {
        throw new Error(`expect object but got ${data} instead`);
    }
    let entries = data as { [key: string]: unknown };
    if ("Ok" in entries) {
        return { kind: "Ok", value: decode_value(entries["Ok"]) };
    } else if ("Err" in entries) {
        return { kind: "Err", message: d_string(entries["Err"]) };
    } else {
        throw new Error("Invalid");
    }
}
