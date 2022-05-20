import {
    decode_array,
    decode_memb,
    decode_number,
    decode_number_pair,
    decode_opt,
    decode_string,
} from "src/decode/decode";
import type { Point2d } from "src/geom2d/geom2d";
import type Color from "./Color";
import type ImageProps from "./ImageProps";

export type ArrowProps = {
    kind: "Arrow2d";
    from: Point2d;
    to: Point2d;
};

export type Point2dProps = {
    kind: "Point2d";
    point: Point2d;
};
export type Shape2dProps = ArrowProps | Point2dProps;
export type Shape2dGroupProps = {
    shapes: Shape2dProps[];
    color: Color;
    line_width: number;
};

type Graphics2dProps = {
    kind: "Graphics2d";
    image: null | ImageProps;
    shape_groups: Shape2dGroupProps[];
    xrange: [number, number];
    yrange: [number, number];
};

export default Graphics2dProps;

export function decode_graphics2d(data: unknown): Graphics2dProps {
    let image = decode_opt(decode_memb(data, "image"), decode_image);
    let shape_groups = decode_array(
        decode_memb(data, "shape_groups"),
        decode_shape_group
    );
    let xrange = decode_number_pair(decode_memb(data, "xrange"));
    let yrange = decode_number_pair(decode_memb(data, "yrange"));
    return { kind: "Graphics2d", image, shape_groups, xrange, yrange };
}

function decode_image(raw: unknown): ImageProps {
    let kind = decode_string(decode_memb(raw, "kind"));
    switch (kind) {
        case "Binary28":
            const rows_unknown = decode_memb(raw, "rows");
            return {
                kind: "Binary28",
                rows: decode_array(rows_unknown, decode_number),
            };
        default:
            throw new Error("TODO");
    }
}

function decode_shape_group(data: unknown): Shape2dGroupProps {
    throw new Error("TODO");
}

function decode_pixels(data: unknown): [number, number, number][][] {
    return decode_array(data, decode_pixel_row);
}

function decode_pixel_row(data: unknown): [number, number, number][] {
    return decode_array(data, decode_pixel);
}

function decode_pixel(data: unknown): [number, number, number] {
    let array = decode_array(data, decode_number);
    if (array.length !== 3) {
        throw new Error("TODO");
    }
    return array as [number, number, number];
}
