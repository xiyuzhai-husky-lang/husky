use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct IllFormedItemSynNodeDecl {
    #[id]
    pub syn_node_path: IllFormedItemSynNodePath,
}

impl From<IllFormedItemSynNodeDecl> for ItemSynNodeDecl {
    fn from(decl: IllFormedItemSynNodeDecl) -> Self {
        ItemSynNodeDecl::AssociatedItem(decl.into())
    }
}

impl HasSynNodeDecl for IllFormedItemSynNodePath {
    type NodeDecl = IllFormedItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        todo!()
    }
}
