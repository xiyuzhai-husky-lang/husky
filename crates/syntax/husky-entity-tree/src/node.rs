use crate::*;
use husky_accessibility::Accessibility;
use husky_entity_path::EntityPathData;
use husky_entity_taxonomy::EntityClass;
use husky_word::Identifier;
use idx_arena::ArenaIdx;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityNode {
    pub visibility: Accessibility,
    pub entity_taxonomy: EntityClass,
    pub ast_idx: AstIdx,
    pub ident: Identifier,
    pub path: EntityPath,
}

impl EntityNode {
    pub(crate) fn entity_taxonomy(&self) -> EntityClass {
        self.entity_taxonomy
    }
}

pub type EntityNodeArena = Arena<EntityNode>;
pub type EntityNodeIdx = ArenaIdx<EntityNode>;
pub type EntityNodeIdxRange = ArenaIdxRange<EntityNode>;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_node(db: &dyn EntityTreeDb, entity_path: EntityPath) -> EntityNode {
    todo!()
}
