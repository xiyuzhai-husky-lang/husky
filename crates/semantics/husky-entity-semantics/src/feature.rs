use crate::*;
use husky_ast::AstIter;
use husky_semantics_error::SemanticResult;
use husky_word::Paradigm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainDefn {
    pub defn_repr: DefinitionRepr,
    pub file: PathItd,
}

impl EntityDefnVariant {
    pub(crate) fn feature(
        db: &dyn EntityDefnQueryGroup,
        entity_path: EntityPathItd,
        paradigm: Paradigm,
        ty: Ty,
        children: Option<AstIter>,
        arena: &ExprArena,
        file: PathItd,
    ) -> SemanticResult<EntityDefnVariant> {
        todo!()
        // Ok(EntityDefnVariant::Feature {
        //     defn_repr: parse_definition_repr(db, paradigm, route, ty, arena, children, file)?,
        // })
    }
}
