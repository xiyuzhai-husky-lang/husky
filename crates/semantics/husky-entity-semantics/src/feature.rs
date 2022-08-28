use crate::*;
use husky_ast::{AstIter, RawExprArena};
use husky_semantics_error::SemanticResult;
use husky_word::Paradigm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainDefn {
    pub defn_repr: DefinitionRepr,
    pub file: FilePtr,
}

impl EntityDefnVariant {
    pub(crate) fn feature(
        db: &dyn EntityDefnQueryGroup,
        route: EntityRoutePtr,
        paradigm: Paradigm,
        ty: RangedEntityRoute,
        children: Option<AstIter>,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnVariant> {
        Ok(EntityDefnVariant::Feature {
            defn_repr: parse_definition_repr(db, paradigm, route, ty, arena, children, file)?,
        })
    }
}
