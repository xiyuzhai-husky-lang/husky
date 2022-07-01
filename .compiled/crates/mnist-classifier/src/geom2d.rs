use crate::*;

pub struct Point2d {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Point2d {
    pub(crate) fn vector(&self) -> crate::geom2d::Vector2d {
        return crate::geom2d::Vector2d::__call__(self.x, self.y);
    }

    pub(crate) fn to(&self, other: &crate::geom2d::Point2d) -> crate::geom2d::Vector2d {
        return crate::geom2d::Vector2d::__call__(other.x - self.x, other.y - self.y);
    }

    pub(crate) fn norm(&self) -> f32 {
        return (self.x * self.x + self.y * self.y).sqrt();
    }

    pub(crate) fn dist(&self, other: &crate::geom2d::Point2d) -> f32 {
        return self.to(&other).norm();
    }
}
pub struct Vector2d {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Vector2d {
    pub(crate) fn point(&self) -> crate::geom2d::Point2d {
        return crate::geom2d::Point2d::__call__(self.x, self.y);
    }

    pub(crate) fn to(&self, other: &crate::geom2d::Vector2d) -> crate::geom2d::Vector2d {
        return crate::geom2d::Vector2d::__call__(other.x - self.x, other.y - self.y);
    }

    pub(crate) fn norm(&self) -> f32 {
        return (self.x * self.x + self.y * self.y).sqrt();
    }

    pub(crate) fn dot(&self, other: &crate::geom2d::Vector2d) -> f32 {
        return self.x * other.x + self.y * other.y;
    }

    pub(crate) fn cross(&self, other: &crate::geom2d::Vector2d) -> f32 {
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
            return (self.y.sgnx() as f32) * cos_value.acos() * 180f32 / 3.1415925f32;
        }
    }

    pub(crate) fn rotation_direction_to(&self, other: &crate::geom2d::Vector2d) -> i32 {
        let cross = self.cross(&other);
        return cross.sgnx();
    }

    pub(crate) fn angle_to(
        &self,
        other: &crate::geom2d::Vector2d,
        is_branch_cut_positive: bool,
    ) -> f32 {
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
            return arc_angle * 180f32 / 3.1415925f32;
        }
    }
}
