use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermCurryPattern {
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
