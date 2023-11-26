use super::*;

#[salsa::interned(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct IllFormedItemSynNodePath {}

impl From<IllFormedItemSynNodePath> for ItemSynNodePath {
    fn from(path: IllFormedItemSynNodePath) -> Self {
        ItemSynNodePath::AssociatedItem(path.into())
    }
}

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub(crate) struct IllFormedItemSynNode {
    #[id]
    pub syn_node_path: IllFormedItemSynNodePath,
}
