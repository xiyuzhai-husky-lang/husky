use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssociatedItem {}

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
