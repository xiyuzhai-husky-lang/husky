use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ConnectedAvailableTraitItemsTable<'a> {
    prelude_trait_items_table: &'a [(Ident, SmallVec<[AvailableTraitItemRecord; 2]>)],
    module_specific_trait_items_table: &'a [(Ident, SmallVec<[AvailableTraitItemRecord; 2]>)],
}
