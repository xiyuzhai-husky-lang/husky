use super::*;

#[salsa::interned(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct IllFormedItemSynNodePath {}

impl From<IllFormedItemSynNodePath> for EntitySynNodePath {
    fn from(path: IllFormedItemSynNodePath) -> Self {
        EntitySynNodePath::AssociatedItem(path.into())
    }
}

impl IllFormedItemSynNodePath {
    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        todo!()
    }
}

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct IllFormedItemSynNode {
    #[id]
    pub syn_node_path: IllFormedItemSynNodePath,
}
