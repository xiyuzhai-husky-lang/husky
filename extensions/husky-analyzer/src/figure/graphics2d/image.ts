import {
    decode_array,
    decode_memb,
    decode_number,
    decode_number_pair,
    decode_opt,
    decode_string,
} from "src/decode/decode";

type ImageProps =
    | {
          kind: "Colored";
          pixels: [number, number, number][][];
      }
    | { kind: "Binary28"; rows: number[] };
export default ImageProps;

export class ImageLoader {
    props: ImageProps;
    constructor(props: ImageProps) {
        this.props = props;
    }

    height(): number {
        switch (this.props.kind) {
            case "Binary28":
                return 28;
            case "Colored":
                throw new Error("TODO");
        }
    }

    width(): number {
        switch (this.props.kind) {
            case "Binary28":
                return 28;
            case "Colored":
                throw new Error("TODO");
        }
    }

    r(i: number, j: number): number {
        switch (this.props.kind) {
            case "Binary28":
                return binary28_row_pixel(this.props.rows[i], j);
            case "Colored":
                throw new Error("TODO");
        }
    }

    g(i: number, j: number): number {
        switch (this.props.kind) {
            case "Binary28":
                return binary28_row_pixel(this.props.rows[i], j);
            case "Colored":
                throw new Error("TODO");
        }
    }

    b(i: number, j: number): number {
        switch (this.props.kind) {
            case "Binary28":
                return binary28_row_pixel(this.props.rows[i], j);
            case "Colored":
                throw new Error("TODO");
        }
    }
}

function binary28_row_pixel(row: number, j: number): number {
    return ((row >> (28 - j)) & 1) * 255;
}

export function decode_image(raw: unknown): ImageProps {
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
