use crate::*;
use ast::{AstIter, RawExprArena};
use semantics_error::SemanticResult;
use semantics_lazy::LazyStmt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainDefn {
    pub stmts: Arc<Vec<Arc<LazyStmt>>>,
    pub file: FilePtr,
}

impl EntityDefnKind {
    pub(crate) fn feature(
        db: &dyn EntityQueryGroup,
        ty: RangedScope,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnKind> {
        let lazy_stmts = semantics_lazy::parse_lazy_stmts(&[], db.upcast(), arena, children, file)?;
        // let feature_block = FeatureBlock::new(db, lazy_stmts, &[], db.features());
        Ok(EntityDefnKind::Feature { ty, lazy_stmts })
    }
}
