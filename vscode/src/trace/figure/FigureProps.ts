import { decode_member_old, decode_string } from "src/decode/decode";
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

export type BlankProps = { kind: "Blank" };

type FigureProps = GalleryProps | Graphics2dProps | Plot2dProps | BlankProps;
export default FigureProps;

export function decode_figure_props(data: unknown): FigureProps {
    let type = decode_member_old(data, "kind", decode_string);
    switch (type) {
        case "Graphics2d":
            return decode_graphics2d(data);
        case "Blank":
            return { kind: "Blank" };
        default:
            console.log(type);
            throw new Error("Todo");
    }
}
