use crate::*;

pub struct TermApplicationPattern {
    m: TermPatternItd,
    n: TermPatternItd,
}

impl TermApplicationPattern {
    pub fn m(&self) -> TermPatternItd {
        self.m
    }
    pub fn n(&self) -> TermPatternItd {
        self.n
    }
}
