import {
    decode_array,
    decode_memb,
    decode_number,
    decode_number_pair,
    decode_opt,
    decode_string,
} from "src/decode/decode";
import type ImageProps from "./graphics2d/image";
import { decode_image } from "./graphics2d/image";
import {
    decode_shape2d_group_props,
    type Shape2dGroupProps,
} from "./graphics2d/shape";

export type Graphics2dProps = {
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
        decode_shape2d_group_props
    );
    let xrange = decode_number_pair(decode_memb(data, "xrange"));
    let yrange = decode_number_pair(decode_memb(data, "yrange"));
    return { kind: "Graphics2d", image, shape_groups, xrange, yrange };
}
