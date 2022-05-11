import { decode_array, decode_memb, decode_number } from "src/decode/decode";

type BinaryImage28 = {
    kind: "BinaryImage28";
    padded_rows: number[];
};

export default BinaryImage28;

export function decode_binary_image28(data: unknown): BinaryImage28 {
    let padded_rows = decode_array(
        decode_memb(data, "padded_rows"),
        decode_number
    );
    return { kind: "BinaryImage28", padded_rows };
}
