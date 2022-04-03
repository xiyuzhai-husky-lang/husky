type Result<T> = { kind: "Ok"; value: T } | { kind: "Err="; msg: String };
export default Result;

export function decode_result<T>(
    data: unknown,
    decode_value: (raw: unknown) => T
): Result<T> {
    throw new Error("TODO");
}
