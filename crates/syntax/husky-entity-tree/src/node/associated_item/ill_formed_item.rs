use super::*;

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IllFormedItemSynNodePath(ItemSynNodePathId);

impl From<IllFormedItemSynNodePath> for ItemSynNodePath {
    fn from(path: IllFormedItemSynNodePath) -> Self {
        ItemSynNodePath::AssocItem(path.into())
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct IllFormedItemSynNode {
    syn_node_path: IllFormedItemSynNodePath,
    ast_idx: AstIdx,
    ident: Ident,
    item_kind: TraitItemKind,
    visibility: Scope,
    is_generic: bool,
}
