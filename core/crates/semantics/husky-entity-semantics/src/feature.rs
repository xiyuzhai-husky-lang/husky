use crate::*;
use husky_ast::{AstIter, RawExprArena};
use husky_lazy_semantics::LazyStmt;
use semantics_error::SemanticResult;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainDefn {
    pub defn_repr: DefinitionRepr,
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
        let stmts = husky_lazy_semantics::parse_lazy_stmts(db.upcast(), arena, children, file, ty)?;
        // let feature_block = FeatureBlock::new(db, lazy_stmts, &[], db.features());
        Ok(EntityDefnVariant::Feature {
            ty,
            defn_repr: DefinitionRepr::LazyBlock { stmts, ty },
        })
    }
}
