use crate::*;

pub struct TermCurryPattern {
    curry_kind: TermCurryVariant,
    x: TermPatternItd,
    y: TermPatternItd,
}
