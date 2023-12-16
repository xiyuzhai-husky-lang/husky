use super::*;

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcaveComponent {
    pub line_segment_sketch: Leash<crate::line_segment_sketch::LineSegmentSketch>,
    pub strokes: CyclicSliceLeashed<crate::line_segment_sketch::LineSegmentStroke>,
}

impl ConcaveComponent {
    pub fn __constructor(line_segment_sketch: Leash<crate::line_segment_sketch::LineSegmentSketch>, strokes: CyclicSliceLeashed<crate::line_segment_sketch::LineSegmentStroke>) -> Self {
        Self{
            line_segment_sketch,
            strokes,
        }
    }
}

#[rustfmt::skip]
pub fn find_concave_components(line_segment_sketch: Leash<crate::line_segment_sketch::LineSegmentSketch>) -> Vec<crate::line_segment_sketch::concave_component::ConcaveComponent> {
    let mut concave_components: Vec<crate::line_segment_sketch::concave_component::ConcaveComponent> = vec![];
    let L = line_segment_sketch.strokes.ilen();
    let mut start = 0;
    let mut end = 1;
    while start > -L && !crate::line_segment_sketch::convexity::is_convex(line_segment_sketch, start) {
        start -= 1
    }
    let ccv_start = start;
    while start < ccv_start + L {
        while end <= start + L && !crate::line_segment_sketch::convexity::is_convex(line_segment_sketch, end) {
            end += 1
        }
        if end > start + 1 {
            concave_components.push(crate::line_segment_sketch::concave_component::ConcaveComponent::__constructor(line_segment_sketch, line_segment_sketch.strokes.cyclic_slice_leashed(start, end)))
        }
        start = end;
        end = start + 1
    }
    return concave_components;
}

#[rustfmt::skip]
impl crate::line_segment_sketch::concave_component::ConcaveComponent {
    #[ad_hoc_task_dependency::memoized_field(15)]
pub fn norm(&'static self) -> f32 {
        self.hausdorff_norm()
    }

    #[ad_hoc_task_dependency::memoized_field(16)]
pub fn rel_norm(&'static self) -> f32 {
        self.norm() / self.displacement().norm()
    }

    #[ad_hoc_task_dependency::memoized_field(17)]
pub fn hausdorff_norm(&'static self) -> f32 {
        let mut hausdorff_norm = 0.0f32;
        let curve_start = &self.strokes.first().unwrap().start;
        let curve_ls = self.line_segment();
        let dp_norm = curve_ls.displacement().norm();
        for i in self.strokes.start()..self.strokes.end() {
            let point = &self.strokes[i as usize].end;
            let point_dist = curve_ls.dist_to_point(&point);
            if point_dist > hausdorff_norm {
                hausdorff_norm = point_dist
            }
        }
        return hausdorff_norm;
    }

    #[ad_hoc_task_dependency::memoized_field(18)]
pub fn angle_change(&'static self) -> f32 {
        let mut angle_change = 0.0f32;
        let mut dp0 = self.strokes[self.strokes.start() as usize].displacement();
        for i in (self.strokes.start() + 1)..self.strokes.end() {
            let dp = self.strokes[i as usize].displacement();
            angle_change += dp0.angle_to(&dp, true);
            dp0 = dp
        }
        return angle_change;
    }

    #[ad_hoc_task_dependency::memoized_field_return_ref(19)]
pub fn bounding_box(&'static self) -> crate::geom2d::BoundingBox {
        let start_point = &self.strokes.first().unwrap().start;
        let mut xmin = start_point.x.into_inner();
        let mut xmax = start_point.x.into_inner();
        let mut ymin = start_point.y.into_inner();
        let mut ymax = start_point.y.into_inner();
        for i in self.strokes.start()..self.strokes.end() {
            let point = &self.strokes[i as usize].end;
            xmin = xmin.min(point.x.into_inner());
            xmax = xmax.max(point.x.into_inner());
            ymin = ymin.min(point.y.into_inner());
            ymax = ymax.max(point.y.into_inner())
        }
        return crate::geom2d::BoundingBox::__constructor(crate::geom2d::ClosedRange::__constructor(xmin, xmax), crate::geom2d::ClosedRange::__constructor(ymin, ymax));
    }

    #[ad_hoc_task_dependency::memoized_field_return_ref(20)]
pub fn relative_bounding_box(&'static self) -> crate::geom2d::RelativeBoundingBox {
        self.line_segment_sketch.bounding_box().relative_bounding_box(&self.bounding_box())
    }

    pub fn line_segment(&self) -> crate::line_segment_sketch::line_segment::LineSegment {
        crate::line_segment_sketch::line_segment::LineSegment::__constructor(self.strokes.first().unwrap().start.clone(), self.strokes.last().unwrap().end.clone())
    }

    pub fn start(&self) -> crate::geom2d::Point2d {
        self.strokes.first().unwrap().start.clone()
    }

    pub fn end(&self) -> crate::geom2d::Point2d {
        self.strokes.last().unwrap().end.clone()
    }

    pub fn displacement(&self) -> crate::geom2d::Vector2d {
        self.line_segment().displacement()
    }

    pub fn start_tangent(&self) -> crate::geom2d::Vector2d {
        self.strokes.first().unwrap().displacement()
    }

    pub fn end_tangent(&self) -> crate::geom2d::Vector2d {
        self.strokes.last().unwrap().displacement()
    }
}