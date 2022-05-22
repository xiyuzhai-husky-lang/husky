import {
    decode_array,
    decode_memb,
    decode_number,
    decode_number_pair,
    decode_opt,
    decode_string,
} from "src/decode/decode";
import type { ImageLayerProps } from "./graphics2d/image";
import { decode_image } from "./graphics2d/image";
import { decode_shape2d_props, type Shape2dProps } from "./graphics2d/shape";

export type Graphics2dProps = {
    kind: "Graphics2d";
    image_layers: ImageLayerProps[];
    shapes: Shape2dProps[];
    xrange: [number, number];
    yrange: [number, number];
};

export default Graphics2dProps;

export function decode_graphics2d(data: unknown): Graphics2dProps {
    let image_layers = decode_array(
        decode_memb(data, "image_layers"),
        decode_image
    );
    let shapes = decode_array(
        decode_memb(data, "shapes"),
        decode_shape2d_props
    );
    let xrange = decode_number_pair(decode_memb(data, "xrange"));
    let yrange = decode_number_pair(decode_memb(data, "yrange"));
    return { kind: "Graphics2d", image_layers, shapes, xrange, yrange };
}
