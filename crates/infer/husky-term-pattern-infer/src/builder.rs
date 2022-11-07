use crate::*;
use husky_expr_syntax::RawExprArena;

pub struct TermPatternInferBuilder<'a> {
    arena: &'a RawExprArena,
}
