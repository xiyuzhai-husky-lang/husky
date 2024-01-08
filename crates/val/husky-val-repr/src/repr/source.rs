use super::*;
use husky_hir_lazy_expr::{HirLazyExprIdx, HirLazyStmtIdx};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValReprSource {
    ValItem(FugitivePath),
    Expansion {
        parent_val_repr: ValRepr,
        source: ValReprExpansionSource,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValReprExpansionSource {
    LetVariable {
        stmt: HirLazyStmtIdx,
    },
    RequireDefault {
        stmt: HirLazyStmtIdx,
    },
    RequireCondition {
        stmt: HirLazyStmtIdx,
    },
    AssertCondition {
        stmt: HirLazyStmtIdx,
    },
    IfCondition {
        stmt: HirLazyStmtIdx,
    },
    ElifCondition {
        stmt: HirLazyStmtIdx,
        branch_idx: u8,
    },
    Expr {
        expr: HirLazyExprIdx,
    },
    Stmt {
        stmt: HirLazyStmtIdx,
    },
}

impl ValReprSource {
    pub(crate) fn caching_class(self) -> ValCachingClass {
        match self {
            ValReprSource::ValItem(_) => ValCachingClass::ValItem,
            ValReprSource::Expansion { source, .. } => source.caching_class(),
        }
    }
}

impl ValReprExpansionSource {
    pub(crate) fn caching_class(self) -> ValCachingClass {
        match self {
            ValReprExpansionSource::LetVariable { .. } => ValCachingClass::Variable,
            ValReprExpansionSource::RequireDefault { .. } => ValCachingClass::Expr,
            ValReprExpansionSource::RequireCondition { .. }
            | ValReprExpansionSource::AssertCondition { .. }
            | ValReprExpansionSource::IfCondition { .. }
            | ValReprExpansionSource::ElifCondition { .. } => ValCachingClass::Condition,
            ValReprExpansionSource::Expr { .. } => ValCachingClass::Expr,
            ValReprExpansionSource::Stmt { .. } => ValCachingClass::Stmt,
        }
    }
}
