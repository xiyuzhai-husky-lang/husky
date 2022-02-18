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

export function get_point_distance(a: Point2d, b: Point2d): number {
    return vector_norm(get_displacement(a, b));
}
export function get_displacement(a: Point2d, b: Point2d): Vector2d {
    return { x: b.x - a.x, y: b.y - a.y };
}

export function vector_norm(v: Vector2d): number {
    return Math.sqrt(v.x * v.x + v.y * v.y);
}

export function get_angle(v: Vector2d): number {
    return Math.atan2(v.y, v.x);
}

export function get_angle_in_degree(v: Vector2d): number {
    return (Math.atan2(v.y, v.x) / Math.PI) * 180;
}
