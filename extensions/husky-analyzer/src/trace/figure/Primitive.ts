import { d_memb_old, decode_string, decode_memb } from "src/decode/decode";

type PrimitiveValueVisualProps = {
    kind: "Primitive";
    value: PrimitiveValue;
};

type PrimitiveValue = {
    kind: "I32" | "B32" | "B64" | "Bool" | "Void";
    value: String;
};

export function decode_primitive_value(data: unknown): PrimitiveValue {
    let kind = d_memb_old(data, "kind", decode_string);
    let value = d_memb_old(data, "value", decode_string);
    switch (kind) {
        case "I32":
        case "B32":
        case "B64":
        case "Bool":
        case "Void":
            return { kind, value };
        default:
            throw new Error();
    }
}

export function decode_primitive_value_visual(
    data: unknown
): PrimitiveValueVisualProps {
    return {
        kind: "Primitive",
        value: decode_primitive_value(decode_memb(data, "value")),
    };
}

export default PrimitiveValueVisualProps;
