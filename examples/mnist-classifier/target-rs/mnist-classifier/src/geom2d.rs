use crate::*;

pub struct Point2d {
    pub x: f32,
    pub y: f32,
} 

pub struct RelativePoint2d {
    pub x: f32,
    pub y: f32,
} 

pub struct Vector2d {
    pub x: f32,
    pub y: f32,
} 

pub struct ClosedRange {
    pub min: f32,
    pub max: f32,
} 

pub struct BoundingBox {
    pub xrange: ClosedRange,
    pub yrange: ClosedRange,
} 

pub struct RelativeBoundingBox {
    pub xrange: ClosedRange,
    pub yrange: ClosedRange,
} 

impl Point2d {
    pub fn from_i_shift28(i: i32, shift: i32) -> Point2d {
        Point2d::__constructor((29 - shift) as f32, (29 - i) as f32)
    }

    pub fn vector(self) -> Vector2d {
        Vector2d::__constructor(self.x, self.y)
    }

    pub fn to(self, other: &Point2d) -> Vector2d {
        Vector2d::__constructor(other.x - self.x, other.y - self.y)
    }

    pub fn norm(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dist(self, other: &Point2d) -> f32 {
        self.to(&other).norm()
    }
}

impl Vector2d {
    pub fn point(self) -> Point2d {
        Point2d::__constructor(self.x, self.y)
    }

    pub fn to(self, other: &Vector2d) -> Vector2d {
        Vector2d::__constructor(other.x - self.x, other.y - self.y)
    }

    pub fn norm(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dot(self, other: &Vector2d) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(self, other: &Vector2d) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn angle(self, is_branch_cut_positive: bool) -> f32 {
        let cos_value = (self.x / self.norm()).min(1);
        if cos_value + 1 < 0.001 {
            if is_branch_cut_positive {
                180
            } else {
                -180
            }
        } else {
            self.y.sgnx() as f32 * cos_value.acos() * 180 / 3.1415925
        }
    }

    pub fn rotation_direction_to(self, other: &Vector2d) -> i32 {
        self.cross(&other).sgnx()
    }

    pub fn angle_to(self, other: &Vector2d, is_branch_cut_positive: bool) -> f32 {
        let self_norm = self.norm();
        assert!(self_norm > 0);
        let other_norm = other.norm();
        assert!(other_norm > 0);
        let cos_value = (self.dot(&other) / (self_norm * other_norm)).min(1);
        if cos_value + 1 < 0.001 {
            if is_branch_cut_positive {
                180
            } else {
                -180
            }
        } else {
            let arc_angle = self.rotation_direction_to(&other) as f32 * cos_value.acos();
            arc_angle * 180 / 3.1415925
        }
    }
}

impl ClosedRange {
    pub fn relative_range(self, other: &ClosedRange) -> ClosedRange {
        assert!(self.max > self.min);
        let span = self.max - self.min;
        let rel_min = (other.min - self.min) / span;
        let rel_max = (other.max - self.min) / span;
        ClosedRange::__constructor(rel_min, rel_max)
    }

    pub fn relative_point(self, v: f32) -> f32 {
        let span = self.max - self.min;
        (v - self.min) / span
    }
}

impl BoundingBox {
    pub fn relative_bounding_box(self, other: &BoundingBox) -> RelativeBoundingBox {
        RelativeBoundingBox::__constructor(self.xrange.relative_range(&other.xrange), self.yrange.relative_range(&other.yrange))
    }

    pub fn relative_point(self, point: &Point2d) -> RelativePoint2d {
        RelativePoint2d::__constructor(self.xrange.relative_point(point.x), self.yrange.relative_point(point.x))
    }

    pub fn xmin(self) -> f32 {
        self.xrange.min
    }

    pub fn xmax(self) -> f32 {
        self.xrange.max
    }

    pub fn ymin(self) -> f32 {
        self.yrange.min
    }

    pub fn ymax(self) -> f32 {
        self.yrange.max
    }
}

impl RelativeBoundingBox {
    pub fn xmin(self) -> f32 {
        self.xrange.min
    }

    pub fn xmax(self) -> f32 {
        self.xrange.max
    }

    pub fn ymin(self) -> f32 {
        self.yrange.min
    }

    pub fn ymax(self) -> f32 {
        self.yrange.max
    }
}