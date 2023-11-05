
fn one_fermi_match {
    fermi_match( major_concave_components, vec![ downmost, upmost, hat]);
}

pub fn upmost() {
    let dp = cc.displacement(); require!( dp.y>0)
    dp.y;
}

pub fn downmost() {
    let dp = cc.displacement(); require!( dp.y<=0)
    - cc.end().y;
}

pub fn hat() {
    let dp = cc.displacement(); require!( dp.y<0) require!( dp.x<0)
    - dp.y- dp.x;
}