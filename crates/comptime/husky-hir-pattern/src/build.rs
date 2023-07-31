use crate::*;

pub trait BuildHirPattern {
    fn hir_pattern_db(&self) -> &dyn HirPatternDb;
    fn syn_pattern_expr_arena(&self) -> &SynPatternExprArena;
}
