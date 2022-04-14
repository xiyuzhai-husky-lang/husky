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

impl EntityDefnVariant {
    pub(crate) fn feature(
        db: &dyn EntityDefnQueryGroup,
        ty: RangedEntityRoute,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnVariant> {
        let lazy_stmts = semantics_lazy::parse_lazy_stmts(&[], db.upcast(), arena, children, file)?;
        // let feature_block = FeatureBlock::new(db, lazy_stmts, &[], db.features());
        Ok(EntityDefnVariant::Feature { ty, lazy_stmts })
    }
}
