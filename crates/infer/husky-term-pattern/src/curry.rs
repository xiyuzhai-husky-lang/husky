use crate::*;

pub struct TermCurryPattern {
    curry_kind: TermCurryVariant,
    x: TermPatternIdx,
    y: TermPatternIdx,
}
