use crate::*;
use ast::{AstIter, RawExprArena};
use semantics_error::SemanticResult;
use semantics_lazy::LazyStmt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Main {
    pub stmts: Arc<Vec<Arc<LazyStmt>>>,
}

impl EntityKind {
    pub(crate) fn feature(
        db: &dyn EntityQueryGroup,
        ty: RangedScope,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityKind> {
        let stmts = semantics_lazy::parse_lazy_stmts(&[], db.upcast(), arena, children, file)?;
        Ok(EntityKind::Feature { ty, stmts })
    }
}
