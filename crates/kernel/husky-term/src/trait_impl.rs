use crate::*;

#[salsa::interned(jar = TermJar)]
pub struct TermTraitImpl {
    ty: Term,
    trai: Trai,
}
