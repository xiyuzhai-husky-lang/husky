use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LineSegment {
    pub(crate) start: crate::geom2d::Point2d,
    pub(crate) end: crate::geom2d::Point2d,
}

impl LineSegment {
    pub(crate) fn __call__(start: crate::geom2d::Point2d, end: crate::geom2d::Point2d) -> Self {
        Self { start, end }
    }
    pub(crate) fn displacement(&self) -> crate::geom2d::Vector2d {
        return self.start.to(&self.end);
    }

    pub(crate) fn dist_to_point(&self, pt: &crate::geom2d::Point2d) -> f32 {
        let ab = self.displacement();
        let ap = self.start.to(&pt);
        if ab.dot(&ap) < 0f32 {
            return ap.norm();
        } else {
            let bp = self.end.to(&pt);
            if ab.dot(&bp) > 0f32 {
                return bp.norm();
            } else {
                return ab.cross(&ap).abs() / ab.norm();
            }
        }
    }
}

impl __StaticInfo for LineSegment {
    type __StaticSelf = LineSegment;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::line_segment_sketch::line_segment::LineSegment".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}
