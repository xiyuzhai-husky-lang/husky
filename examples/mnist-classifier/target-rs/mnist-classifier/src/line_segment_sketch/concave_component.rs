use super::*;

pub struct ConcaveComponent{
    line_segment_sketch: Leash<LineSegmentSketch>,
    strokes: Leash<CyclicSlice<LineSegmentStroke>>,
}

pub fn find_concave_components(line_segment_sketch: Leash<LineSegmentSketch>) -> Vec<ConcaveComponent> {
    let mut concave_components: Vec<ConcaveComponent> = vec![];
    let L = line_segment_sketch.strokes.ilen();
    let mut start = 0;
    let mut end = 1;
    while start > -L && !is_convex(line_segment_sketch, start) {
        start-= 1
    }
    let ccv_start = start;
    while start < ccv_start + L {
        while end <= start + L && !is_convex(line_segment_sketch, end) {
            end+= 1
        }
        if end > start + 1 {
            concave_components.push(ConcaveComponent(line_segment_sketch, line_segment_sketch.strokes.cyclic_slice_leashed(start, end)))
        }
        start = end;
        end = start + 1
    }
    return concave_components;
}

impl Visualize for ConcaveComponent {
    fn visualize(self) -> Html {
        self.strokes.visualize()
    }
}

impl ConcaveComponent {
    pub fn norm(self) -> f32 {
        self.hausdorff_norm
    }

    pub fn rel_norm(self) -> f32 {
        self.norm / self.displacement().norm()
    }

    pub fn hausdorff_norm(self) -> f32 {
        let mut hausdorff_norm = 0;
        let curve_start = self.strokes.first().unwrap().start;
        let curve_ls = self.line_segment();
        let dp_norm = curve_ls.displacement().norm();
        for i in self.strokes.start()..self.strokes.end() {
            let point = self.strokes[i].end;
            let point_dist = curve_ls.dist_to_point(point);
            if point_dist > hausdorff_norm {
                hausdorff_norm = point_dist
            }
        }
        return hausdorff_norm;
    }

    pub fn angle_change(self) -> f32 {
        let mut angle_change = 0;
        let mut dp0 = self.strokes[self.strokes.start()].displacement();
        for i in (self.strokes.start() + 1)..self.strokes.end() {
            let dp = self.strokes[i].displacement();
            angle_change += dp0.angle_to(dp, true);
            dp0 = dp
        }
        return angle_change;
    }

    pub fn bounding_box(self) -> BoundingBox {
        let start_point = self.strokes.first().unwrap().start;
        let mut xmin = start_point.x;
        let mut xmax = start_point.x;
        let mut ymin = start_point.y;
        let mut ymax = start_point.y;
        for i in self.strokes.start()..self.strokes.end() {
            let point = self.strokes[i].end;
            xmin = xmin.min(point.x);
            xmax = xmax.max(point.x);
            ymin = ymin.min(point.y);
            ymax = ymax.max(point.y)
        }
        return BoundingBox(ClosedRange(xmin, xmax), ClosedRange(ymin, ymax));
    }

    pub fn relative_bounding_box(self) -> RelativeBoundingBox {
        self.line_segment_sketch.bounding_box.relative_bounding_box(self.bounding_box)
    }

    pub fn line_segment(self) -> LineSegment {
        LineSegment(self.strokes.first().unwrap().start.clone(), self.strokes.last().unwrap().end.clone())
    }

    pub fn start(self) -> Point2d {
        self.strokes.first().unwrap().start.clone()
    }

    pub fn end(self) -> Point2d {
        self.strokes.last().unwrap().end.clone()
    }

    pub fn displacement(self) -> Vector2d {
        self.line_segment().displacement()
    }

    pub fn start_tangent(self) -> Vector2d {
        self.strokes.first().unwrap().displacement()
    }

    pub fn end_tangent(self) -> Vector2d {
        self.strokes.last().unwrap().displacement()
    }
}