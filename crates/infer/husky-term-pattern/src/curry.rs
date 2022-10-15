use crate::*;

pub struct TermCurryPattern<'a> {
    curry_kind: TermCurryKind,
    x: &'a TermPattern<'a>,
    y: &'a TermPattern<'a>,
}
