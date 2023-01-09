use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssociatedItem {
    impl_block_idx: ImplBlockIdx,
    impl_block_kind: ImplBlockKind,
    ident: Identifier,
}

impl AssociatedItem {
    pub fn impl_block_idx(&self) -> ImplBlockIdx {
        self.impl_block_idx
    }

    pub fn impl_block_kind(&self) -> ImplBlockKind {
        self.impl_block_kind
    }

    pub fn ident(&self) -> Identifier {
        self.ident
    }
}

impl AsVecMapEntry for AssociatedItem {
    type K = AssociatedItemPath;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        todo!()
    }

    fn key_ref(&self) -> &Self::K {
        todo!()
    }
}

pub type AssociatedItemArena = Arena<AssociatedItem>;
pub type AssociatedItemIdx = ArenaIdx<AssociatedItem>;
pub type AssociatedItemIdxRange = ArenaIdxRange<AssociatedItem>;
