use super::*;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq)]
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
    let L = line_segment_sketch.deleash().strokes.ilen();
    let mut start = 0;
    let mut end = 1;
    while start > -L && !crate::line_segment_sketch::convexity::is_convex(&line_segment_sketch.deleash(), start) {
        start -= 1
    }
    let ccv_start = start;
    while start < ccv_start + L {
        while end <= start + L && !crate::line_segment_sketch::convexity::is_convex(&line_segment_sketch.deleash(), end) {
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
impl Visualize for crate::line_segment_sketch::concave_component::ConcaveComponent {
    fn visualize(&self, __visual_synchrotron: &mut __VisualSynchrotron) -> husky_core::visual::Visual {
        self.strokes.deleash().visualize(__visual_synchrotron)
    }
}

#[rustfmt::skip]
impl crate::line_segment_sketch::concave_component::ConcaveComponent {
    #[ad_hoc_devsoul_dependency::memo(ingredient_index = 15)]
    pub fn norm(&'static self) -> f32 {
        <crate::line_segment_sketch::concave_component::ConcaveComponent>::hausdorff_norm(__self)
    }

    #[ad_hoc_devsoul_dependency::memo(ingredient_index = 16)]
    pub fn rel_norm(&'static self) -> f32 {
        <crate::line_segment_sketch::concave_component::ConcaveComponent>::norm(__self) / __self.deleash().displacement().norm()
    }

    #[ad_hoc_devsoul_dependency::memo(ingredient_index = 17)]
    pub fn hausdorff_norm(&'static self) -> f32 {
        let mut hausdorff_norm = 0.0f32;
        let curve_start = __self.deleash().strokes.deleash().first().unwrap().deleash().start;
        let curve_ls = __self.deleash().line_segment();
        let dp_norm = curve_ls.displacement().norm();
        for i in __self.deleash().strokes.deleash().start()..__self.deleash().strokes.deleash().end() {
            let point = __self.deleash().strokes[i as usize].end;
            let point_dist = curve_ls.dist_to_point(point.deleash());
            if point_dist > hausdorff_norm {
                hausdorff_norm = point_dist
            }
        }
        return hausdorff_norm;
    }

    #[ad_hoc_devsoul_dependency::memo(ingredient_index = 18)]
    pub fn angle_change(&'static self) -> f32 {
        let mut angle_change = 0.0f32;
        let mut dp0 = __self.deleash().strokes[__self.deleash().strokes.deleash().start() as usize].displacement();
        for i in (__self.deleash().strokes.deleash().start() + 1)..__self.deleash().strokes.deleash().end() {
            let dp = __self.deleash().strokes[i as usize].displacement();
            angle_change += dp0.angle_to(&dp, true);
            dp0 = dp
        }
        return angle_change;
    }

    #[ad_hoc_devsoul_dependency::memo(ingredient_index = 19, return_leash)]
    pub fn bounding_box(&'static self) -> crate::geom2d::BoundingBox {
        let start_point = __self.deleash().strokes.deleash().first().unwrap().deleash().start;
        let mut xmin = start_point.deleash().x;
        let mut xmax = start_point.deleash().x;
        let mut ymin = start_point.deleash().y;
        let mut ymax = start_point.deleash().y;
        for i in __self.deleash().strokes.deleash().start()..__self.deleash().strokes.deleash().end() {
            let point = __self.deleash().strokes[i as usize].end;
            xmin = xmin.min(point.deleash().x);
            xmax = xmax.max(point.deleash().x);
            ymin = ymin.min(point.deleash().y);
            ymax = ymax.max(point.deleash().y)
        }
        return crate::geom2d::BoundingBox::__constructor(crate::geom2d::ClosedRange::__constructor(xmin, xmax), crate::geom2d::ClosedRange::__constructor(ymin, ymax));
    }

    #[ad_hoc_devsoul_dependency::memo(ingredient_index = 20, return_leash)]
    pub fn relative_bounding_box(&'static self) -> crate::geom2d::RelativeBoundingBox {
        <crate::line_segment_sketch::LineSegmentSketch>::bounding_box(__self.line_segment_sketch.deleash()).deleash().relative_bounding_box(<crate::line_segment_sketch::concave_component::ConcaveComponent>::bounding_box(__self).deleash())
    }

    pub fn line_segment(&self) -> crate::line_segment_sketch::line_segment::LineSegment {
        crate::line_segment_sketch::line_segment::LineSegment::__constructor(self.strokes.deleash().first().unwrap().deleash().start.clone(), self.strokes.deleash().last().unwrap().deleash().end.clone())
    }

    pub fn start(&self) -> crate::geom2d::Point2d {
        self.strokes.deleash().first().unwrap().deleash().start.clone()
    }

    pub fn end(&self) -> crate::geom2d::Point2d {
        self.strokes.deleash().last().unwrap().deleash().end.clone()
    }

    pub fn displacement(&self) -> crate::geom2d::Vector2d {
        self.line_segment().displacement()
    }

    pub fn start_tangent(&self) -> crate::geom2d::Vector2d {
        self.strokes.deleash().first().unwrap().deleash().displacement()
    }

    pub fn end_tangent(&self) -> crate::geom2d::Vector2d {
        self.strokes.deleash().last().unwrap().deleash().displacement()
    }
}