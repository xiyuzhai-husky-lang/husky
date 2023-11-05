
pub fn is_convex(, ) {
    let L = line_segment_sketch.strokes.ilen();
    let current_displacement = line_segment_sketch.strokes[ index% L].displacement();
    let previous_displacement = line_segment_sketch.strokes[ index-1% L].displacement();
    let is_rotation_counterclockwise_result = previous_displacement.rotation_direction_to( current_displacement);
    if is_rotation_counterclockwise_result==0 {
        let previous_raw_cross =-999999;
        let previous_interval = line_segment_sketch.strokes[ index-1% L].points; for {
            let displacement = line_segment_sketch.contour.displacement( previous_interval.start(), i1);
            previous_raw_cross= previous_raw_cross.max( current_displacement.cross( displacement));
        }
        let current_raw_cross =-999999;
        let current_interval = line_segment_sketch.strokes[ index% L].points; for {
            let displacement = line_segment_sketch.contour.displacement( previous_interval.start(), i2);
            current_raw_cross= current_raw_cross.max( current_displacement.cross( displacement));
        } return( current_raw_cross< previous_raw_cross)
    } else { return( is_rotation_counterclockwise_result>0)
    }
}