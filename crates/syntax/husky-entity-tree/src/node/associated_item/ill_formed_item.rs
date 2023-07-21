use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct IllFormedItemSynNodePath {}

impl From<IllFormedItemSynNodePath> for EntitySynNodePath {
    fn from(path: IllFormedItemSynNodePath) -> Self {
        EntitySynNodePath::AssociatedItem(path.into())
    }
}

impl IllFormedItemSynNodePath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        todo!()
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct IllFormedItemNode {
    #[id]
    pub node_path: IllFormedItemSynNodePath,
}
