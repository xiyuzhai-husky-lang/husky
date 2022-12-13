use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Point2d {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Point2d {
    pub(crate) fn __call__(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub(crate) fn from_i_shift28(i: i32, shift: i32) -> Point2d {
        return Point2d::__call__((29 - shift) as f32, (29 - i) as f32);
    }
    pub(crate) fn vector(&self) -> Vector2d {
        return Vector2d::__call__(self.x, self.y);
    }

    pub(crate) fn to(&self, other: &Point2d) -> Vector2d {
        return Vector2d::__call__(other.x - self.x, other.y - self.y);
    }

    pub(crate) fn norm(&self) -> f32 {
        return (self.x * self.x + self.y * self.y).sqrt();
    }

    pub(crate) fn dist(&self, other: &Point2d) -> f32 {
        return self.to(&other).norm();
    }
}

impl __StaticInfo for Point2d {
    type __StaticSelf = Point2d;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::geom2d::Point2d".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RelativePoint2d {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl RelativePoint2d {
    pub(crate) fn __call__(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl __StaticInfo for RelativePoint2d {
    type __StaticSelf = RelativePoint2d;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::geom2d::RelativePoint2d".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Vector2d {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Vector2d {
    pub(crate) fn __call__(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub(crate) fn point(&self) -> Point2d {
        return Point2d::__call__(self.x, self.y);
    }

    pub(crate) fn to(&self, other: &Vector2d) -> Vector2d {
        return Vector2d::__call__(other.x - self.x, other.y - self.y);
    }

    pub(crate) fn norm(&self) -> f32 {
        return (self.x * self.x + self.y * self.y).sqrt();
    }

    pub(crate) fn dot(&self, other: &Vector2d) -> f32 {
        return self.x * other.x + self.y * other.y;
    }

    pub(crate) fn cross(&self, other: &Vector2d) -> f32 {
        return self.x * other.y - self.y * other.x;
    }

    pub(crate) fn angle(&self, is_branch_cut_positive: bool) -> f32 {
        let cos_value = (self.x / self.norm()).min(1f32);
        if cos_value + 1f32 < 0.001f32 {
            if is_branch_cut_positive {
                return 180f32;
            } else {
                return -180f32;
            }
        } else {
            return (self.y.sgnx() as f32) * cos_value.acos() * 180f32 / 3.1415926f32;
        }
    }

    pub(crate) fn rotation_direction_to(&self, other: &Vector2d) -> i32 {
        let cross = self.cross(&other);
        return cross.sgnx();
    }

    pub(crate) fn angle_to(&self, other: &Vector2d, is_branch_cut_positive: bool) -> f32 {
        let this_norm = self.norm();
        assert!(this_norm > 0f32);
        let other_norm = other.norm();
        assert!(other_norm > 0f32);
        let cos_value = (self.dot(&other) / (this_norm * other_norm)).min(1f32);
        if cos_value + 1f32 < 0.001f32 {
            if is_branch_cut_positive {
                return 180f32;
            } else {
                return -180f32;
            }
        } else {
            let arc_angle = (self.rotation_direction_to(&other) as f32) * cos_value.acos();
            return arc_angle * 180f32 / 3.1415926f32;
        }
    }
}

impl __StaticInfo for Vector2d {
    type __StaticSelf = Vector2d;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::geom2d::Vector2d".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ClosedRange {
    pub(crate) min: f32,
    pub(crate) max: f32,
}

impl ClosedRange {
    pub(crate) fn __call__(min: f32, max: f32) -> Self {
        Self { min, max }
    }
    pub(crate) fn relative_range(&self, other: &ClosedRange) -> ClosedRange {
        assert!(self.max > self.min);
        let span = self.max - self.min;
        let rel_min = (other.min - self.min) / span;
        let rel_max = (other.max - self.min) / span;
        return ClosedRange::__call__(rel_min, rel_max);
    }

    pub(crate) fn relative_point(&self, v: f32) -> f32 {
        let span = self.max - self.min;
        return (v - self.min) / span;
    }
}

impl __StaticInfo for ClosedRange {
    type __StaticSelf = ClosedRange;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::geom2d::ClosedRange".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BoundingBox {
    pub(crate) xrange: ClosedRange,
    pub(crate) yrange: ClosedRange,
}

impl BoundingBox {
    pub(crate) fn __call__(xrange: ClosedRange, yrange: ClosedRange) -> Self {
        Self { xrange, yrange }
    }
    pub(crate) fn relative_bounding_box(&self, other: &BoundingBox) -> RelativeBoundingBox {
        return RelativeBoundingBox::__call__(
            self.xrange.relative_range(&other.xrange),
            self.yrange.relative_range(&other.yrange),
        );
    }

    pub(crate) fn relative_point(&self, point: &Point2d) -> RelativePoint2d {
        return RelativePoint2d::__call__(
            self.xrange.relative_point(point.x),
            self.yrange.relative_point(point.x),
        );
    }

    pub(crate) fn xmin(&self) -> f32 {
        return self.xrange.min;
    }

    pub(crate) fn xmax(&self) -> f32 {
        return self.xrange.max;
    }

    pub(crate) fn ymin(&self) -> f32 {
        return self.yrange.min;
    }

    pub(crate) fn ymax(&self) -> f32 {
        return self.yrange.max;
    }
}

impl __StaticInfo for BoundingBox {
    type __StaticSelf = BoundingBox;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::geom2d::BoundingBox".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RelativeBoundingBox {
    pub(crate) xrange: ClosedRange,
    pub(crate) yrange: ClosedRange,
}

impl RelativeBoundingBox {
    pub(crate) fn __call__(xrange: ClosedRange, yrange: ClosedRange) -> Self {
        Self { xrange, yrange }
    }
    pub(crate) fn xmin(&self) -> f32 {
        return self.xrange.min;
    }

    pub(crate) fn xmax(&self) -> f32 {
        return self.xrange.max;
    }

    pub(crate) fn ymin(&self) -> f32 {
        return self.yrange.min;
    }

    pub(crate) fn ymax(&self) -> f32 {
        return self.yrange.max;
    }
}

impl __StaticInfo for RelativeBoundingBox {
    type __StaticSelf = RelativeBoundingBox;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::geom2d::RelativeBoundingBox".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}
