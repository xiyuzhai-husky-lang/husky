use super::*;

pub struct TraitsInUse {
    prelude_traits: TraitOrderedSet,
    module_specific_traits: TraitOrderedSet,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct InUseTraitItemsTable {
    prelude_trait_items_table: InUseTraitItemsTableImpl,
    module_specific_trait_items_table: InUseTraitItemsTableImpl,
}

type InUseTraitItemsTableImpl = SmallVecPairMap<Ident, SmallVec<[InUseTraitItemRecord; 2]>, 16>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct InUseTraitItemRecord {
    path: TraitItemPath,
    scope: Scope,
}

impl InUseTraitItemRecord {
    pub fn path(&self) -> TraitItemPath {
        self.path
    }
}

// section: query

/// available trait items with given identifier
/// designed for
#[derive(Debug, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct InUseTraitItemsWithGivenIdent<'a> {
    prelude_trait_items: Option<&'a [InUseTraitItemRecord]>,
    module_specific_trait_items: Option<&'a [InUseTraitItemRecord]>,
}

impl InUseTraitItemsTable {
    fn available_trait_items_with_given_ident<'a>(
        &'a self,
        ident: Ident,
    ) -> InUseTraitItemsWithGivenIdent<'a> {
        InUseTraitItemsWithGivenIdent {
            prelude_trait_items: self.prelude_trait_items_table.get_value(ident),
            module_specific_trait_items: self.module_specific_trait_items_table.get_value(ident),
        }
    }
}

impl<'a> InUseTraitItemsWithGivenIdent<'a> {
    pub fn records(self) -> impl Iterator<Item = InUseTraitItemRecord> + 'a {
        self.module_specific_trait_items
            .into_iter()
            .map(|arr| arr.iter().copied())
            .flatten()
            .chain(
                self.prelude_trait_items
                    .into_iter()
                    .map(|arr| arr.iter().copied())
                    .flatten(),
            )
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn trai_item_table(db: &dyn EntityTreeDb, traits: TraitOrderedSet) -> InUseTraitItemsTableImpl {
    todo!()
}
