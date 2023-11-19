use super::*;

pub struct LineSegment{
    start: Point2d,
    end: Point2d,
}

impl LineSegment {
    fn displacement(self) -> Vector2d {
        self.start.to(self.end)
    }

    fn dist_to_point(self, pt: Point2d) -> f32 {
        let ab = self.displacement();
        let ap = self.start.to(pt);
        if ab.dot(ap) < 0 {
            ap.norm()
        } else {
            let bp = self.end.to(pt);
            if ab.dot(bp) > 0 {
                bp.norm()
            } else {
                ab.cross(ap).abs() / ab.norm()
            }
        }
    }
}