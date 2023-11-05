struct!ConnectedComponentDistribution{row_start:i32, row_end:i32, upper_mass:i32, lower_mass:i32}struct!EffHoles{matches:Vec<Option<Leash<RawContour>>>}pub!fn!hole_tmpl{
    let!len=ct.contour_len;require!(len>4)
    len+0;
}
struct!ConnectedComponent{mask:BinaryImage28}pub!fn!horizontal_extend{
    let!y=a|x|x<<1|x>>1;
    let!z=a|y|y<<1|y>>1;while!z!=y{
        y=z;
        z=a|y|y<<1|y>>1;
    }
return!(y)
}
pub!fn!find_connected_components{
    let!result:Vec<ConnectedComponent>=vec![];
    let!unsearched=img.clone();for!{while!unsearched[j]{
            let!a=unsearched[j];
            let!shift=a.ctz();
            let!mask=new_zeros();
            mask[j]=horizontal_extend(a, 1<<shift);
            let!flag=false;while!!flag{
                flag=true;
                let!i=j;for!{
                    let!old_row=mask[i+1];
                    let!new_row=old_row|horizontal_extend(img[i+1], mask[i]);                    if!!new_row{
                        break!;
                    }
;                    if!old_row!=new_row{
                        flag=false;
                        mask[i+1]=new_row;
                    }
;
                }
for!{
                    let!old_row=mask[i];
                    let!new_row=old_row|horizontal_extend(img[i], mask[i+1]);                    if!old_row!=new_row{
                        flag=false;
                        mask[i]=new_row;
                    }
;
                }
            }
for!{
                unsearched[k]&=!mask[k];
            }
            result.push(ConnectedComponent(mask));
        }
    }
return!(result)
}
impl!fn!visualize(){
    Self.mask.visualize();
}
impl!fn!raw_contours(self){
    find_raw_contours(Self);
}
fn!eff_holes(self){
    let!raw_contours=Self.raw_contours.collect_leashes();
    let!matches:Vec<Option<Leash<RawContour>>>=vec![];
    raw_contours.pop_with_largest_opt_f32(hole_tmpl);
    matches.push(raw_contours.pop_with_largest_opt_f32(hole_tmpl));
    matches.push(raw_contours.pop_with_largest_opt_f32(hole_tmpl));return!(EffHoles(matches))
}
fn!max_hole_ilen(self){
    let!max_hole_ilen=0;
    let!raw_contours=Self.raw_contours;for!{
        let!hole_ilen=raw_contours[i].points.ilen();        if!max_hole_ilen<hole_ilen{
            max_hole_ilen=hole_ilen;
        }
;
    }
return!(max_hole_ilenasf32)
}
fn!max_row_span(self){
    let!max_row:i32=0;for!{
        max_row=max_row.max(Self.mask[i].span());
    }
return!(max_rowasf32)
}
fn!row_span_sum(self){
    let!row_span_sum=0;for!{
        row_span_sum+=Self.mask[i].span();
    }
return!(row_span_sumasf32)
}
fn!distribution(self){
    let!row_start=1;for!{        if!Self.mask[row_start]{
            break!;
        }
;
    }
    let!row_end=row_start+1;for!{        if!!Self.mask[row_end]{
            break!;
        }
;
    }
    let!height=row_end-row_start;
    let!half_height=height/2;
    let!upper_mass=0;for!{
        upper_mass+=Self.mask[i1].co();
    }
    let!lower_mass=0;for!{
        lower_mass+=Self.mask[i2].co();
    }
return!(ConnectedComponentDistribution(row_start, row_end, upper_mass, lower_mass))
}
fn!upper_mass(self){
    Self.distribution.upper_massasf32;
}
fn!lower_mass(self){
    Self.distribution.lower_massasf32;
}
fn!top_k_row_span_sum(, ){
    let!top_k_row_span_sum=0;assert!(k>0)
    let!i=1;for!{        if!Self.mask[i]{
            break!;
        }
;
    }
for!{
        top_k_row_span_sum+=Self.mask[j].span();
    }
return!(top_k_row_span_sumasf32)
}
fn!top_k_row_right_mass_sum(, ){
    let!top_k_row_span_sum=0;assert!(k>0)
    let!i=1;for!{        if!Self.mask[i]{
            break!;
        }
;
    }
for!{
        top_k_row_span_sum+=Self.mask[j].right_mass();
    }
return!(top_k_row_span_sumasf32)
}
