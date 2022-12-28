use crate::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ExprSheet {
    expr_arena: ExprArena,
    pattern_expr_arena: PatternExprArena,
}
