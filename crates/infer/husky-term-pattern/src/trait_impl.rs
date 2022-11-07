use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermTraitImplPattern {
    ty: TermPatternItd,
    trai: TermPatternItd,
}

impl TermTraitImplPattern {
    pub fn ty(&self) -> TermPatternItd {
        self.ty
    }

    pub fn trai(&self) -> TermPatternItd {
        self.trai
    }
}
