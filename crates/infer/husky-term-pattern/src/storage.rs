use crate::*;

pub struct TermPatternInterner {
    patterns: Vec<TermPattern>,
}

pub struct TermPatternIdx(usize);
