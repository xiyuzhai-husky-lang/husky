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
