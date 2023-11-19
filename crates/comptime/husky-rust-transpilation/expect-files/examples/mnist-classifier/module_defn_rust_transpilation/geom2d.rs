
struct Point2d{
    x: f32,
    y: f32,
}

struct RelativePoint2d{
    x: f32,
    y: f32,
}

struct Vector2d{
    x: f32,
    y: f32,
}

struct ClosedRange{
    min: f32,
    max: f32,
}

struct BoundingBox{
    xrange: ClosedRange,
    yrange: ClosedRange,
}

struct RelativeBoundingBox{
    xrange: ClosedRange,
    yrange: ClosedRange,
}