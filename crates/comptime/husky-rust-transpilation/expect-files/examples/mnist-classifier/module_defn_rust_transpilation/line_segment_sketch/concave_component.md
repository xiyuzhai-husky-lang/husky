struct!ConcaveComponent{line_segment_sketch:Leash<LineSegmentSketch>, strokes:Leash<CyclicSlice<LineSegmentStroke>>}pub!fn!find_concave_components{
    let!concave_components:Vec<ConcaveComponent>=vec![];
    let!L=line_segment_sketch.strokes.ilen();
    let!start=0;
    let!end=1;while!start>-L&&!is_convex(line_segment_sketch, start){
        start-= 1;
    }
    let!ccv_start=start;while!start<ccv_start+L{while!end<=start+L&&!is_convex(line_segment_sketch, end){
            end+= 1;
        }
        if!end>start+1{
            concave_components.push(ConcaveComponent(line_segment_sketch, line_segment_sketch.strokes.cyclic_slice_leashed(start, end)));
        }
;
        start=end;
        end=start+1;
    }
return!(concave_components)
}
impl!fn!visualize(){
    Self.strokes.visualize();
}
impl!fn!norm(self){
    Self.hausdorff_norm;
}
fn!rel_norm(self){
    Self.norm/Self.displacement().norm();
}
fn!hausdorff_norm(self){
    let!hausdorff_norm=0;
    let!curve_start=Self.strokes.first().unwrap().start;
    let!curve_ls=Self.line_segment();
    let!dp_norm=curve_ls.displacement().norm();for!{
        let!point=Self.strokes[i].end;
        let!point_dist=curve_ls.dist_to_point(point);        if!point_dist>hausdorff_norm{
            hausdorff_norm=point_dist;
        }
;
    }
return!(hausdorff_norm)
}
fn!angle_change(self){
    let!angle_change=0;
    let!dp0=Self.strokes[Self.strokes.start()].displacement();for!{
        let!dp=Self.strokes[i].displacement();
        angle_change+=dp0.angle_to(dp, true);
        dp0=dp;
    }
return!(angle_change)
}
fn!bounding_box(self){
    let!start_point=Self.strokes.first().unwrap().start;
    let!xmin=start_point.x;
    let!xmax=start_point.x;
    let!ymin=start_point.y;
    let!ymax=start_point.y;for!{
        let!point=Self.strokes[i].end;
        xmin=xmin.min(point.x);
        xmax=xmax.max(point.x);
        ymin=ymin.min(point.y);
        ymax=ymax.max(point.y);
    }
return!(BoundingBox(ClosedRange(xmin, xmax), ClosedRange(ymin, ymax)))
}
fn!relative_bounding_box(self){
    Self.line_segment_sketch.bounding_box.relative_bounding_box(Self.bounding_box);
}
fn!line_segment(){
    LineSegment(Self.strokes.first().unwrap().start.clone(), Self.strokes.last().unwrap().end.clone());
}
fn!start(){
    Self.strokes.first().unwrap().start.clone();
}
fn!end(){
    Self.strokes.last().unwrap().end.clone();
}
fn!displacement(){
    Self.line_segment().displacement();
}
fn!start_tangent(){
    Self.strokes.first().unwrap().displacement();
}
fn!end_tangent(){
    Self.strokes.last().unwrap().displacement();
}
