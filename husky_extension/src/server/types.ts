import type { Point2d, Vector2d } from "./geom2d/types";
export type Trace = { idx: number; parent: number | null; tokens: Token[] };
export type Token = { type: string; value: string; spaces_before?: number };
export type FigureProps = GalleryProps | Graphics2dProps | Plot2dProps | null;

export type GalleryProps = { type: "Gallery" };

export type Graphics2dProps = {
    type: "Graphics2d";
    image: null | ImageProps;
    shape_groups: ShapeGroupProps[];
    xrange: [number, number];
    yrange: [number, number];
};
export type ImageProps = {
    data: number[];
    originalWidth: number;
    originalHeight: number;
};
export type ShapeGroupProps = {
    shapes: ShapeProps[];
    color: Color;
    lineWidth: number;
};
export type ShapeProps = ArrowProps;
export type ArrowProps = { shape_kind: "Arrow"; from: Point2d; to: Point2d };

export type Plot2dProps = {
    type: "Plot2d";
    plot_kind: "Scatter";
    groups: PointGroup[];
    xrange: [number, number];
    yrange: [number, number];
};
export type PointGroup = { points: Point2d[]; color: Color };

export type Color = "red" | "yellow" | "green" | "blue";
