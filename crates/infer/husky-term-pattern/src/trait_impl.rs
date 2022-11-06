use crate::*;

pub struct TermTraitImplPattern<'a> {
    ty: &'a TermPattern<'a>,
    trai: &'a TermPattern<'a>,
}
