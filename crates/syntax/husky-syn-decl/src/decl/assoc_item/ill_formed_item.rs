use super::*;
use husky_entity_tree::node::assoc_item::ill_formed_item::IllFormedItemSynNodePath;

#[salsa::tracked]
pub struct IllFormedItemSynNodeDecl {
    #[id]
    pub syn_node_path: IllFormedItemSynNodePath,
    pub syn_expr_region: SynExprRegion,
}

impl From<IllFormedItemSynNodeDecl> for ItemSynNodeDecl {
    fn from(decl: IllFormedItemSynNodeDecl) -> Self {
        ItemSynNodeDecl::AssocItem(decl.into())
    }
}

impl HasSynNodeDecl for IllFormedItemSynNodePath {
    type NodeDecl = IllFormedItemSynNodeDecl;

    fn syn_node_decl<'a>(self, _db: &'a ::salsa::Db) -> Self::NodeDecl {
        todo!()
    }
}
