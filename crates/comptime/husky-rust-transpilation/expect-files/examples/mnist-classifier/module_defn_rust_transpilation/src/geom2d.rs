use crate::*;

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq)]
pub struct Point2d {
    pub x: f32,
    pub y: f32,
}

impl Point2d {
    pub fn __constructor(x: f32, y: f32) -> Self {
        Self{
            x,
            y,
        }
    }
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq)]
pub struct RelativePoint2d {
    pub x: f32,
    pub y: f32,
}

impl RelativePoint2d {
    pub fn __constructor(x: f32, y: f32) -> Self {
        Self{
            x,
            y,
        }
    }
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq)]
pub struct Vector2d {
    pub x: f32,
    pub y: f32,
}

impl Vector2d {
    pub fn __constructor(x: f32, y: f32) -> Self {
        Self{
            x,
            y,
        }
    }
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq)]
pub struct ClosedRange {
    pub min: f32,
    pub max: f32,
}

impl ClosedRange {
    pub fn __constructor(min: f32, max: f32) -> Self {
        Self{
            min,
            max,
        }
    }
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq)]
pub struct BoundingBox {
    pub xrange: crate::geom2d::ClosedRange,
    pub yrange: crate::geom2d::ClosedRange,
}

impl BoundingBox {
    pub fn __constructor(xrange: crate::geom2d::ClosedRange, yrange: crate::geom2d::ClosedRange) -> Self {
        Self{
            xrange,
            yrange,
        }
    }
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq)]
pub struct RelativeBoundingBox {
    pub xrange: crate::geom2d::ClosedRange,
    pub yrange: crate::geom2d::ClosedRange,
}

impl RelativeBoundingBox {
    pub fn __constructor(xrange: crate::geom2d::ClosedRange, yrange: crate::geom2d::ClosedRange) -> Self {
        Self{
            xrange,
            yrange,
        }
    }
}

#[rustfmt::skip]
impl crate::geom2d::Point2d {
    pub fn from_i_shift28(i: i32, shift: i32) -> crate::geom2d::Point2d {
        crate::geom2d::Point2d::__constructor((29 - shift) as f32, (29 - i) as f32)
    }

    pub fn vector(&self) -> crate::geom2d::Vector2d {
        crate::geom2d::Vector2d::__constructor(self.x, self.y)
    }

    pub fn to(&self, other: &crate::geom2d::Point2d) -> crate::geom2d::Vector2d {
        crate::geom2d::Vector2d::__constructor(other.x - self.x, other.y - self.y)
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dist(&self, other: &crate::geom2d::Point2d) -> f32 {
        self.to(other).norm()
    }
}

#[rustfmt::skip]
impl crate::geom2d::Vector2d {
    pub fn point(&self) -> crate::geom2d::Point2d {
        crate::geom2d::Point2d::__constructor(self.x, self.y)
    }

    pub fn to(&self, other: &crate::geom2d::Vector2d) -> crate::geom2d::Vector2d {
        crate::geom2d::Vector2d::__constructor(other.x - self.x, other.y - self.y)
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dot(&self, other: &crate::geom2d::Vector2d) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &crate::geom2d::Vector2d) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn angle(&self, is_branch_cut_positive: bool) -> f32 {
        let cos_value = (self.x / self.norm()).min(1.0f32);
        if cos_value + 1.0f32 < 0.001f32 {
            if is_branch_cut_positive {
                180.0f32
            } else {
                -180.0f32
            }
        } else {
            self.y.sgnx() as f32 * cos_value.acos() * 180.0f32 / 3.1415926f32
        }
    }

    pub fn rotation_direction_to(&self, other: &crate::geom2d::Vector2d) -> i32 {
        self.cross(other).sgnx()
    }

    pub fn angle_to(&self, other: &crate::geom2d::Vector2d, is_branch_cut_positive: bool) -> f32 {
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

#[rustfmt::skip]
impl crate::geom2d::ClosedRange {
    pub fn relative_range(&self, other: &crate::geom2d::ClosedRange) -> crate::geom2d::ClosedRange {
        assert!(self.max > self.min);
        let span = self.max - self.min;
        let rel_min = (other.min - self.min) / span;
        let rel_max = (other.max - self.min) / span;
        crate::geom2d::ClosedRange::__constructor(rel_min, rel_max)
    }

    pub fn relative_point(&self, v: f32) -> f32 {
        let span = self.max - self.min;
        (v - self.min) / span
    }
}

#[rustfmt::skip]
impl crate::geom2d::BoundingBox {
    pub fn relative_bounding_box(&self, other: &crate::geom2d::BoundingBox) -> crate::geom2d::RelativeBoundingBox {
        crate::geom2d::RelativeBoundingBox::__constructor(self.xrange.relative_range(&other.xrange), self.yrange.relative_range(&other.yrange))
    }

    pub fn relative_point(&self, point: &crate::geom2d::Point2d) -> crate::geom2d::RelativePoint2d {
        crate::geom2d::RelativePoint2d::__constructor(self.xrange.relative_point(point.x), self.yrange.relative_point(point.x))
    }

    pub fn xmin(&self) -> f32 {
        self.xrange.min
    }

    pub fn xmax(&self) -> f32 {
        self.xrange.max
    }

    pub fn ymin(&self) -> f32 {
        self.yrange.min
    }

    pub fn ymax(&self) -> f32 {
        self.yrange.max
    }
}

#[rustfmt::skip]
impl crate::geom2d::RelativeBoundingBox {
    pub fn xmin(&self) -> f32 {
        self.xrange.min
    }

    pub fn xmax(&self) -> f32 {
        self.xrange.max
    }

    pub fn ymin(&self) -> f32 {
        self.yrange.min
    }

    pub fn ymax(&self) -> f32 {
        self.yrange.max
    }
}