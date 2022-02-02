import * as t from "io-ts";

export const tPoint2d = t.interface({
    x: t.number,
    y: t.number,
});
export const tVector2d = t.interface({
    x: t.number,
    y: t.number,
});

export type Point2d = t.TypeOf<typeof tPoint2d>;
export type Vector2d = t.TypeOf<typeof tVector2d>;
