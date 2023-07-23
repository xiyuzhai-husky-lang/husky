use super::*;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct IllFormedItemSynNodeDecl {
    #[id]
    pub syn_node_path: IllFormedItemSynNodePath,
    pub node: IllFormedItemSynNode,
}

impl From<IllFormedItemSynNodeDecl> for SynNodeDecl {
    fn from(decl: IllFormedItemSynNodeDecl) -> Self {
        SynNodeDecl::AssociatedItem(decl.into())
    }
}

impl HasNodeDecl for IllFormedItemSynNodePath {
    type NodeDecl = IllFormedItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        todo!()
    }
}
