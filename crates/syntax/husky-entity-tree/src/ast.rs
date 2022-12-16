use husky_accessibility::Accessibility;
use crate::*;

pub struct EntityAstSheet {
    arena: Arena<EntityAst>,
    implementations: Vec<AstIdx>,
    errors: Vec<EntityTreeError>,
}

pub type EntityTreeNodeIdxRange = ArenaIdxRange<EntityAst>;

pub struct EntityAst {
    idx: AstIdx,
    accessibility: Accessibility,
    kind: EntityKind,
    children: Option<EntityTreeNodeIdxRange>,
}

pub enum EntityAstVariant {
    UseAll { parent: EntityPath },
    Use { path: EntityPath },
    Defn,
}
