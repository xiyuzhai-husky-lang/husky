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
        return Point2d::__call__((29 - shift) as f32, (29 - i) as f32)
    }
    pub(crate) fn vector(&self) -> Vector2d {
        return Vector2d::__call__(self.x, self.y)
    }

    pub(crate) fn to(&self, other: &Point2d) -> Vector2d {
        return Vector2d::__call__(other.x - self.x, other.y - self.y)
    }

    pub(crate) fn norm(&self) -> f32 {
        return (self.x * self.x + self.y * self.y).sqrt()
    }

    pub(crate) fn dist(&self, other: &Point2d) -> f32 {
        return self.to(&other).norm()
    }
}

impl __HasStaticTypeInfo for Point2d {
    type StaticSelf = Point2d;

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::geom2d::Point2d".into()
    }
}

impl<'eval> __AnyValue<'eval> for Point2d {
    fn print_short(&self) -> String {
        todo!()
    }

    fn to_json_value(&self) -> __JsonValue {
        todo!()
    }

    fn short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short {
        todo!()
    }

    fn static_ty() -> __EntityRoutePtr {
        __lazy_entity_route_from_text!("mnist_classifier::geom2d::Point2d")
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
        return Point2d::__call__(self.x, self.y)
    }

    pub(crate) fn to(&self, other: &Vector2d) -> Vector2d {
        return Vector2d::__call__(other.x - self.x, other.y - self.y)
    }

    pub(crate) fn norm(&self) -> f32 {
        return (self.x * self.x + self.y * self.y).sqrt()
    }

    pub(crate) fn dot(&self, other: &Vector2d) -> f32 {
        return self.x * other.x + self.y * other.y
    }

    pub(crate) fn cross(&self, other: &Vector2d) -> f32 {
        return self.x * other.y - self.y * other.x
    }

    pub(crate) fn angle(&self, is_branch_cut_positive: bool) -> f32 {
        let cos_value = (self.x / self.norm()).min(1f32);
        if cos_value + 1f32 < 0.001f32 {
            if is_branch_cut_positive {
                return 180f32
            } else {
                return -180f32
            }
        } else {
            return (self.y.sgnx() as f32) * cos_value.acos() * 180f32 / 3.1415925f32
        }
    }

    pub(crate) fn rotation_direction_to(&self, other: &Vector2d) -> i32 {
        let cross = self.cross(&other);
        return cross.sgnx()
    }

    pub(crate) fn angle_to(&self, other: &Vector2d, is_branch_cut_positive: bool) -> f32 {
        let this_norm = self.norm();
        assert!(this_norm > 0f32);
        let other_norm = other.norm();
        assert!(other_norm > 0f32);
        let cos_value = (self.dot(&other) / (this_norm * other_norm)).min(1f32);
        if cos_value + 1f32 < 0.001f32 {
            if is_branch_cut_positive {
                return 180f32
            } else {
                return -180f32
            }
        } else {
            let arc_angle = (self.rotation_direction_to(&other) as f32) * cos_value.acos();
            return arc_angle * 180f32 / 3.1415925f32
        }
    }
}

impl __HasStaticTypeInfo for Vector2d {
    type StaticSelf = Vector2d;

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::geom2d::Vector2d".into()
    }
}

impl<'eval> __AnyValue<'eval> for Vector2d {
    fn print_short(&self) -> String {
        todo!()
    }

    fn to_json_value(&self) -> __JsonValue {
        todo!()
    }

    fn short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short {
        todo!()
    }

    fn static_ty() -> __EntityRoutePtr {
        __lazy_entity_route_from_text!("mnist_classifier::geom2d::Vector2d")
    }
}
