use crate::*;
use husky_accessibility::Accessibility;
use husky_entity_kind::EntityKind;

pub struct EntityNode {
    visibility: Accessibility,
    entity_kind: EntityKind,
}

impl EntityNode {
    pub(crate) fn entity_kind(&self) -> EntityKind {
        self.entity_kind
    }
}

pub(crate) fn entity_node(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> EntityTreeResult<&EntityNode> {
    let entity_abs_path = absolutize_parent(db, entity_path);
    todo!()
}
