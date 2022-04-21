import {
    decode_member,
    decode_member_old,
    decode_string,
} from "src/decode/decode";
import type { Point2d } from "src/geom2d/geom2d";
import type Graphics2dProps from "./Graphics2d";
import type Color from "./Color";
import { decode_graphics2d } from "./Graphics2d";

export type PointGroup = {
    points: Point2d[];
    color: Color;
};
export type GalleryProps = { kind: "Gallery" };
export type Plot2dProps = {
    kind: "Plot2d";
    plot_kind: "Scatter";
    groups: PointGroup[];
    xrange: [number, number];
    yrange: [number, number];
};

export type PrimitiveValueFigureProps = {
    kind: "Primitive";
    value: PrimitiveValue;
};

type PrimitiveValue = {
    kind: "I32" | "B32" | "B64" | "Bool" | "Void";
    value: String;
};

function decode_primitive_value(data: unknown): PrimitiveValue {
    let kind = decode_member_old(data, "kind", decode_string);
    let value = decode_member_old(data, "value", decode_string);
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

type FigureProps =
    | GalleryProps
    | Graphics2dProps
    | Plot2dProps
    | PrimitiveValueFigureProps;
export default FigureProps;

export function decode_figure_props(data: unknown): FigureProps {
    let type = decode_member_old(data, "kind", decode_string);
    switch (type) {
        case "Graphics2d":
            return decode_graphics2d(data);
        case "Primitive":
            return {
                kind: "Primitive",
                value: decode_primitive_value(decode_member(data, "value")),
            };
        default:
            console.log(type);
            throw new Error("Todo");
    }
}
