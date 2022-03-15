import type { Point2d } from "src/geom2d/geom2d";

export type ImageProps = {
    data: number[];
    original_width: number;
    original_height: number;
};
export type Color = "red" | "yellow" | "green" | "blue";
export type ArrowProps = {
    type: "Arrow";
    from: Point2d;
    to: Point2d;
};
export type ShapeProps = ArrowProps;
export type ShapeGroupProps = {
    shapes: ShapeProps;
    color: Color;
    line_width: number;
};
export type Graphics2dProps = {
    type: "Graphics2d";
    image: null | ImageProps;
    shape_groups: ShapeGroupProps[];
    xrange: [number, number];
    yrange: [number, number];
};
export type GalleryProps = { type: "Gallery" };
export type PointGroup = {
    points: Point2d[];
    color: Color;
};
export type Plot2dProps = {
    type: "Plot2d";
    plot_kind: "Scatter";
    groups: PointGroup[];
    xrange: [number, number];
    yrange: [number, number];
};
export type FigureProps = GalleryProps | Graphics2dProps | Plot2dProps | null;

export default FigureProps;

export function decode_figure_props(data: unknown): FigureProps {
    throw new Error("todo");
}
