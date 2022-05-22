import {
    decode_array,
    decode_memb,
    decode_number,
    decode_number_pair,
    decode_opt,
    decode_string,
} from "src/decode/decode";
import { decode_point2d, type Point2d } from "src/geom2d/geom2d";
import type { Color } from "../color";
import { decode_color } from "../color";

export type Arrow2dProps = {
    kind: "Arrow2d";
    from: Point2d;
    to: Point2d;
};

export type Point2dProps = {
    kind: "Point2d";
    point: Point2d;
};
export type Shape2dProps = Arrow2dProps | Point2dProps;
export type Shape2dGroupProps = {
    shapes: Shape2dProps[];
    color: Color;
    line_width: number;
};
export type Shape2dKind = "Point2d" | "Arrow2d";

// export function decode_shape2d_group_props(data: unknown): Shape2dGroupProps {
//     console.log("data = ", data);
//     const kind = decode_shape2d_kind(decode_memb(data, "kind"));
//     return {
//         shapes: decode_array(decode_memb(data, "shapes"), (data: unknown) =>
//             decode_shape2d_props(data, kind)
//         ),
//         color: decode_color(decode_memb(data, "color")),
//         line_width: decode_number(decode_memb(data, "line_width")),
//     };
// }

function decode_shape2d_kind(data: unknown): Shape2dKind {
    switch (decode_string(decode_memb(data, "kind"))) {
        case "Arrow2d":
            return "Arrow2d";
        case "Point2d":
            return "Point2d";
        default:
            throw new Error("TODO");
    }
}

export function decode_shape2d_props(data: unknown): Shape2dProps {
    const kind = decode_shape2d_kind(data);
    switch (kind) {
        case "Arrow2d":
            return {
                kind: "Arrow2d",
                from: decode_point2d(decode_memb(data, "from")),
                to: decode_point2d(decode_memb(data, "to")),
            };
        case "Point2d":
            return {
                kind: "Point2d",
                point: decode_point2d(decode_memb(data, "point")),
            };
    }
}
