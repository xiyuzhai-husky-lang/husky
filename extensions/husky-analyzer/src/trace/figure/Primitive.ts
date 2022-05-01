import { d_memb_old, d_string } from "src/decode/decode";

type PrimitiveValueFigureProps = {
    kind: "Primitive";
    value: PrimitiveValue;
};

type PrimitiveValue = {
    kind: "I32" | "B32" | "B64" | "Bool" | "Void";
    value: String;
};

export function decode_primitive_value(data: unknown): PrimitiveValue {
    let kind = d_memb_old(data, "kind", d_string);
    let value = d_memb_old(data, "value", d_string);
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

export default PrimitiveValueFigureProps;
