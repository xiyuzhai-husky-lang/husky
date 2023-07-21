use super::*;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct IllFormedItemNodeDecl {
    #[id]
    pub node_path: IllFormedItemNodePath,
    pub node: IllFormedItemNode,
}

impl From<IllFormedItemNodeDecl> for NodeDecl {
    fn from(decl: IllFormedItemNodeDecl) -> Self {
        NodeDecl::AssociatedItem(decl.into())
    }
}

impl HasNodeDecl for IllFormedItemNodePath {
    type NodeDecl = IllFormedItemNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        todo!()
    }
}
