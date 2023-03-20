use super::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeAssociatedItem {
    #[id]
    pub id: TypeAssociatedItemId,
    pub impl_block: TypeImplBlock,
    pub ast_idx: AstIdx,
    pub ident: Ident,
    pub associated_item_kind: AssociatedItemKind,
    pub accessibility: Visibility,
    pub is_generic: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct TypeAssociatedItemId {
    impl_block_id: TypeImplBlockId,
}
