use crate::*;
use husky_accessibility::Accessibility;
use husky_entity_card::EntityCard;
use husky_entity_path::EntityPathData;
use husky_word::Identifier;
use idx_arena::ArenaIdx;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityNode {
    pub visibility: Accessibility,
    pub entity_card: EntityCard,
    pub ast_idx: AstIdx,
    pub ident: Identifier,
    pub path: EntityPath,
}

impl EntityNode {
    pub(crate) fn entity_card(&self) -> EntityCard {
        self.entity_card
    }
}

pub type EntityNodeArena = Arena<EntityNode>;
pub type EntityNodeIdx = ArenaIdx<EntityNode>;
pub type EntityNodeIdxRange = ArenaIdxRange<EntityNode>;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_node(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> EntityTreeResult<EntityNode> {
    todo!()
}
