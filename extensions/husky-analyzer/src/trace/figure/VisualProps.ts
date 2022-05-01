import { decode_memb, decode_string } from "src/decode/decode";
import type BinaryImage28 from "./BinaryImage28";
import { decode_binary_image28 } from "./BinaryImage28";
import type PrimitiveValueVisualProps from "./Primitive";
import { decode_primitive_value_visual } from "./Primitive";

type VisualProps = BinaryImage28 | PrimitiveValueVisualProps;

export default VisualProps;

export function decode_visual_props(data: unknown): VisualProps {
    let data_kind = decode_string(decode_memb(data, "kind"));
    switch (data_kind) {
        case "BinaryImage28":
            return decode_binary_image28(data);
        case "Primitive":
            return decode_primitive_value_visual(data);
        default:
            console.log("visual props data: ", data);
            throw new Error("TODO");
    }
}
