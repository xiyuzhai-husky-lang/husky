import * as t from "io-ts";

import { tPoint2d } from "./geom2d/types";

export const tTraceToken = t.interface({
    kind: t.string,
    value: t.string,
    spaces_before: t.union([t.number, t.undefined]),
});
export const tTrace = t.interface({
    id: t.number,
    parent: t.union([t.number, t.null]),
    tokens: t.array(tTraceToken),
});
export const tImageProps = t.interface({
    data: t.array(t.number),
    original_width: t.number,
    original_height: t.number,
});
export const tColor = t.union([
    t.literal("red"),
    t.literal("yellow"),
    t.literal("green"),
    t.literal("blue"),
]);
export const tArrowProps = t.interface({
    type: t.literal("Arrow"),
    from: tPoint2d,
    to: tPoint2d,
});
export const tShapeProps = tArrowProps;
export const tShapeGroupProps = t.interface({
    shapes: t.array(tShapeProps),
    color: tColor,
    line_width: t.number,
});
export const tGraphics2dProps = t.interface({
    type: t.literal("Graphics2d"),
    image: t.union([t.null, tImageProps]),
    shape_groups: t.array(tShapeGroupProps),
    xrange: t.tuple([t.number, t.number]),
    yrange: t.tuple([t.number, t.number]),
});
export const tGalleryProps = t.interface({ type: t.literal("Gallery") });
export const tPointGroup = t.interface({
    points: t.array(tPoint2d),
    color: tColor,
});
export const tPlot2dProps = t.interface({
    type: t.literal("Plot2d"),
    plot_kind: t.literal("Scatter"),
    groups: t.array(tPointGroup),
    xrange: t.tuple([t.number, t.number]),
    yrange: t.tuple([t.number, t.number]),
});
export const tFigureProps = t.union([
    tGalleryProps,
    tGraphics2dProps,
    tPlot2dProps,
    t.null,
]);

export type FigureProps = t.TypeOf<typeof tFigureProps>;
export type Trace = t.TypeOf<typeof tTrace>;
export type TraceToken = t.TypeOf<typeof tTraceToken>;
export type GalleryProps = t.TypeOf<typeof tGalleryProps>;
export type Graphics2dProps = t.TypeOf<typeof tGraphics2dProps>;
export type ImageProps = t.TypeOf<typeof tImageProps>;
export type ShapeGroupProps = t.TypeOf<typeof tShapeGroupProps>;
export type ShapeProps = t.TypeOf<typeof tShapeProps>;
export type Plot2dProps = t.TypeOf<typeof tPlot2dProps>;
export type PointGroup = t.TypeOf<typeof tPointGroup>;
export type Color = t.TypeOf<typeof tColor>;
