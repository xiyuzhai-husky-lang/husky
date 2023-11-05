struct!LineSegment{start:Point2d, end:Point2d}impl!fn!displacement(){
    Self.start.to(Self.end);
}
fn!dist_to_point(, ){
    let!ab=Self.displacement();
    let!ap=Self.start.to(pt);    if!ab.dot(ap)<0{
        ap.norm();
    }
else!{
        let!bp=Self.end.to(pt);        if!ab.dot(bp)>0{
            bp.norm();
        }
else!{
            ab.cross(ap).abs()/ab.norm();
        }
;
    }
;
}
