use crate::*;
use husky_ast::AstIter;
use husky_semantics_error::SemanticResult;
use husky_word::Paradigm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainDefn {
    pub defn_repr: DefinitionRepr,
    pub file: FileItd,
}

impl EntityDefnVariant {
    pub(crate) fn feature(
        db: &dyn EntityDefnQueryGroup,
        route: EntityRoutePtr,
        paradigm: Paradigm,
        ty: RangedEntityRoute,
        children: Option<AstIter>,
        arena: &RawExprArena,
        file: FileItd,
    ) -> SemanticResult<EntityDefnVariant> {
        Ok(EntityDefnVariant::Feature {
            defn_repr: parse_definition_repr(db, paradigm, route, ty, arena, children, file)?,
        })
    }
}
