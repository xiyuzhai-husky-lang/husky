use super::*;

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IllFormedItemSynNodePath(ItemSynNodePathId);
