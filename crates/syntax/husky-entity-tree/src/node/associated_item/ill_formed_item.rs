use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct IllFormedItemNodePath {}

impl From<IllFormedItemNodePath> for EntityNodePath {
    fn from(path: IllFormedItemNodePath) -> Self {
        EntityNodePath::AssociatedItem(path.into())
    }
}

impl IllFormedItemNodePath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        todo!()
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct IllFormedItemNode {
    #[id]
    pub node_path: IllFormedItemNodePath,
}
