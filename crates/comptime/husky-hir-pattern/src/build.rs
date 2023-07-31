use crate::*;
use husky_syn_expr::SynPatternExprArena;

pub trait BuildHirPattern {
    fn hir_pattern_db(&self) -> &dyn HirPatternDb;
    fn syn_pattern_expr_arena(&self) -> &SynPatternExprArena;
}
