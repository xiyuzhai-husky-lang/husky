use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point2d {
    pub x: NotNan<f32>,
    pub y: NotNan<f32>,
}

impl Point2d {
    pub fn __constructor(x: f32, y: f32) -> Self {
        let x = NotNan::new(x).unwrap();
        let y = NotNan::new(y).unwrap();
        Self{
            x,
            y,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelativePoint2d {
    pub x: NotNan<f32>,
    pub y: NotNan<f32>,
}

impl RelativePoint2d {
    pub fn __constructor(x: f32, y: f32) -> Self {
        let x = NotNan::new(x).unwrap();
        let y = NotNan::new(y).unwrap();
        Self{
            x,
            y,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector2d {
    pub x: NotNan<f32>,
    pub y: NotNan<f32>,
}

impl Vector2d {
    pub fn __constructor(x: f32, y: f32) -> Self {
        let x = NotNan::new(x).unwrap();
        let y = NotNan::new(y).unwrap();
        Self{
            x,
            y,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosedRange {
    pub min: NotNan<f32>,
    pub max: NotNan<f32>,
}

impl ClosedRange {
    pub fn __constructor(min: f32, max: f32) -> Self {
        let min = NotNan::new(min).unwrap();
        let max = NotNan::new(max).unwrap();
        Self{
            min,
            max,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundingBox {
    pub xrange: ClosedRange,
    pub yrange: ClosedRange,
}

impl BoundingBox {
    pub fn __constructor(xrange: ClosedRange, yrange: ClosedRange) -> Self {
        Self{
            xrange,
            yrange,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelativeBoundingBox {
    pub xrange: ClosedRange,
    pub yrange: ClosedRange,
}

impl RelativeBoundingBox {
    pub fn __constructor(xrange: ClosedRange, yrange: ClosedRange) -> Self {
        Self{
            xrange,
            yrange,
        }
    }
}

impl Point2d {
    pub fn from_i_shift28(i: i32, shift: i32) -> Point2d {
        Point2d::__constructor((29 - shift) as f32, (29 - i) as f32)
    }

    pub fn vector(&self) -> Vector2d {
        Vector2d::__constructor(self.x.into_inner(), self.y.into_inner())
    }

    pub fn to(&self, other: &Point2d) -> Vector2d {
        Vector2d::__constructor(other.x.into_inner() - self.x.into_inner(), other.y.into_inner() - self.y.into_inner())
    }

    pub fn norm(&self) -> f32 {
        (self.x.into_inner() * self.x.into_inner() + self.y.into_inner() * self.y.into_inner()).sqrt()
    }

    pub fn dist(&self, other: &Point2d) -> f32 {
        self.to(other).norm()
    }
}

impl Vector2d {
    pub fn point(&self) -> Point2d {
        Point2d::__constructor(self.x.into_inner(), self.y.into_inner())
    }

    pub fn to(&self, other: &Vector2d) -> Vector2d {
        Vector2d::__constructor(other.x.into_inner() - self.x.into_inner(), other.y.into_inner() - self.y.into_inner())
    }

    pub fn norm(&self) -> f32 {
        (self.x.into_inner() * self.x.into_inner() + self.y.into_inner() * self.y.into_inner()).sqrt()
    }

    pub fn dot(&self, other: &Vector2d) -> f32 {
        self.x.into_inner() * other.x.into_inner() + self.y.into_inner() * other.y.into_inner()
    }

    pub fn cross(&self, other: &Vector2d) -> f32 {
        self.x.into_inner() * other.y.into_inner() - self.y.into_inner() * other.x.into_inner()
    }

    pub fn angle(&self, is_branch_cut_positive: bool) -> f32 {
        let cos_value = (self.x.into_inner() / self.norm()).min(1.0f32);
        if cos_value + 1.0f32 < 0.001f32 {
            if is_branch_cut_positive {
                180.0f32
            } else {
                -180.0f32
            }
        } else {
            self.y.into_inner().sgnx() as f32 * cos_value.acos() * 180.0f32 / 3.1415926f32
        }
    }

    pub fn rotation_direction_to(&self, other: &Vector2d) -> i32 {
        self.cross(other).sgnx()
    }

    pub fn angle_to(&self, other: &Vector2d, is_branch_cut_positive: bool) -> f32 {
        let self_norm = self.norm();
        assert!(self_norm > 0.0f32);
        let other_norm = other.norm();
        assert!(other_norm > 0.0f32);
        let cos_value = (self.dot(other) / (self_norm * other_norm)).min(1.0f32);
        if cos_value + 1.0f32 < 0.001f32 {
            if is_branch_cut_positive {
                180.0f32
            } else {
                -180.0f32
            }
        } else {
            let arc_angle = self.rotation_direction_to(other) as f32 * cos_value.acos();
            arc_angle * 180.0f32 / 3.1415926f32
        }
    }
}

impl ClosedRange {
    pub fn relative_range(&self, other: &ClosedRange) -> ClosedRange {
        assert!(self.max.into_inner() > self.min.into_inner());
        let span = self.max.into_inner() - self.min.into_inner();
        let rel_min = (other.min.into_inner() - self.min.into_inner()) / span;
        let rel_max = (other.max.into_inner() - self.min.into_inner()) / span;
        ClosedRange::__constructor(rel_min, rel_max)
    }

    pub fn relative_point(&self, v: f32) -> f32 {
        let span = self.max.into_inner() - self.min.into_inner();
        (v - self.min.into_inner()) / span
    }
}

impl BoundingBox {
    pub fn relative_bounding_box(&self, other: &BoundingBox) -> RelativeBoundingBox {
        RelativeBoundingBox::__constructor(self.xrange.relative_range((&other.xrange)), self.yrange.relative_range((&other.yrange)))
    }

    pub fn relative_point(&self, point: &Point2d) -> RelativePoint2d {
        RelativePoint2d::__constructor(self.xrange.relative_point(point.x.into_inner()), self.yrange.relative_point(point.x.into_inner()))
    }

    pub fn xmin(&self) -> f32 {
        self.xrange.min.into_inner()
    }

    pub fn xmax(&self) -> f32 {
        self.xrange.max.into_inner()
    }

    pub fn ymin(&self) -> f32 {
        self.yrange.min.into_inner()
    }

    pub fn ymax(&self) -> f32 {
        self.yrange.max.into_inner()
    }
}

impl RelativeBoundingBox {
    pub fn xmin(&self) -> f32 {
        self.xrange.min.into_inner()
    }

    pub fn xmax(&self) -> f32 {
        self.xrange.max.into_inner()
    }

    pub fn ymin(&self) -> f32 {
        self.yrange.min.into_inner()
    }

    pub fn ymax(&self) -> f32 {
        self.yrange.max.into_inner()
    }
}