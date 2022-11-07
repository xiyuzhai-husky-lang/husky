use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermCurryPattern {
    curry_kind: TermCurryVariant,
    x: TermPatternItd,
    y: TermPatternItd,
}

impl TermCurryPattern {
    pub fn x(&self) -> TermPatternItd {
        self.x
    }
    pub fn y(&self) -> TermPatternItd {
        self.y
    }
}
