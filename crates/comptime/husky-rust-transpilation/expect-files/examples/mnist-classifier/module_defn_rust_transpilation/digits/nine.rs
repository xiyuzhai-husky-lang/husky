
fn nine_match {
    fermi_match( major_concave_components, vec![ downmost]);
}

fn nine_match_refine {
    fermi_match( major_concave_components, vec![ big_cc]);
}

fn is_nine { require!( matches!) require!( matches!)
    let eff_holes = major_connected_component.eff_holes; require!( matches!)
    let down_match = nine_match.matches[0]; require!( matches!)
    let down_match_dp_y = down_match.unwrap().displacement().y;
    let higher_excess = major_connected_component.upper_mass- major_connected_component.lower_mass; require!( higher_excess>7)
    if matches! { require!( major_concave_components.ilen()>=2)
        let nine_match_refine_result = nine_match_refine.matches[0]; require!( matches!) require!( nine_match_refine.norm<1)
        let higher_excess = major_connected_component.upper_mass- major_connected_component.lower_mass;
        let upper_arc = nine_match_refine.matches[0]; require!( matches!) require!( upper_arc.unwrap().displacement().y>0) require!( upper_arc.unwrap().angle_change<-110) require!( nine_match_refine.norm<9)
        let a = major_connected_component.top_k_row_right_mass_sum(3); require!( a<22) require!( a>9) return( OneVsAll :: Yes)
    }
    OneVsAll :: Yes;
}

pub fn downmost {
    let dp = cc.displacement(); require!( dp.y<0)
    dp.y;
}

pub fn big_cc {
    let dp = cc.displacement(); require!( dp.y>0) require!( cc.relative_bounding_box.ymin()>0.4)
    cc.relative_bounding_box.ymin();
}