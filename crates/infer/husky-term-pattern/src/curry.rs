use crate::*;

pub struct TermCurryPattern<'a> {
    curry_kind: TermCurryVariant,
    x: &'a TermPattern<'a>,
    y: &'a TermPattern<'a>,
}
