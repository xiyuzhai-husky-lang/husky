use super::*;
use vec_like::VecMapGetEntry;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AvailableTraitItemsTable<'a> {
    prelude_trait_items_table: &'a [(Ident, SmallVec<[AvailableTraitItemRecord; 2]>)],
    module_specific_trait_items_table: &'a [(Ident, SmallVec<[AvailableTraitItemRecord; 2]>)],
}

impl<'a> AvailableTraitItemsTable<'a> {
    pub fn query(db: &'a ::salsa::Db, module_path: ModulePath) -> Self {
        let toolchain = module_path.toolchain(db);
        Self {
            prelude_trait_items_table: if module_path.crate_path(db)
                == db.vfs_path_menu(toolchain).core_library()
            {
                &[]
            } else {
                non_core_crate_prelude_trait_items_table(db, toolchain)
            },
            module_specific_trait_items_table: module_specific_trait_items_table(db, module_path),
        }
    }

    pub fn available_trait_items_with_given_ident(
        self,
        ident: Ident,
    ) -> Option<AvailableTraitItemsWithGivenIdent<'a>> {
        let items = AvailableTraitItemsWithGivenIdent {
            prelude_trait_items: self
                .prelude_trait_items_table
                .get_entry(ident)
                .map(|(_, records)| records as &[_]),
            module_specific_trait_items: self
                .module_specific_trait_items_table
                .get_entry(ident)
                .map(|(_, records)| records as &[_]),
        };
        if items.prelude_trait_items.is_none() && items.module_specific_trait_items.is_none() {
            return None;
        }
        Some(items)
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn non_core_crate_prelude_trait_items_table(
    db: &::salsa::Db,
    toolchain: Toolchain,
) -> AvailableTraitItemRecords {
    trait_item_records(
        db,
        none_core_crate_universal_prelude(db, toolchain).as_ref(),
    )
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn module_specific_trait_items_table(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> AvailableTraitItemRecords {
    trait_item_records(db, module_path.item_tree_sheet(db).module_symbols())
}

type AvailableTraitItemRecords =
    SmallVecPairMap<Ident, SmallVec<[AvailableTraitItemRecord; 2]>, 16>;

fn trait_item_records(
    db: &::salsa::Db,
    item_symbol_table_ref: EntitySymbolTableRef,
) -> AvailableTraitItemRecords {
    let mut table: SmallVecPairMap<Ident, SmallVec<[AvailableTraitItemRecord; 2]>, 16> =
        Default::default();
    for entry in item_symbol_table_ref.data().iter() {
        let PrincipalEntityPath::MajorItem(MajorItemPath::Trait(trai_path)) =
            entry.symbol().principal_entity_path(db)
        else {
            continue;
        };
        for (ident, trai_item_path) in trai_path.assoc_item_paths(db) {
            let record = AvailableTraitItemRecord {
                trai_symbol: entry.symbol(),
                trai_path,
                trai_item_path: *trai_item_path,
                scope: entry.visibility(),
            };
            table.update_value_or_insert(*ident, |records| records.push(record), smallvec![record])
        }
    }
    table
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AvailableTraitItemRecord {
    trai_symbol: EntitySymbol,
    trai_path: TraitPath,
    trai_item_path: TraitItemPath,
    scope: Scope,
}

impl AvailableTraitItemRecord {
    pub fn trai_path(&self) -> TraitPath {
        self.trai_path
    }

    pub fn trai_item_path(&self) -> TraitItemPath {
        self.trai_item_path
    }
}

// section: query

/// available trait items with given identifier
/// designed for
#[derive(Debug, Clone, Copy)]
#[salsa::derive_debug_with_db]
pub struct AvailableTraitItemsWithGivenIdent<'a> {
    prelude_trait_items: Option<&'a [AvailableTraitItemRecord]>,
    module_specific_trait_items: Option<&'a [AvailableTraitItemRecord]>,
}

impl<'a> AvailableTraitItemsWithGivenIdent<'a> {
    pub fn records(self) -> impl Iterator<Item = AvailableTraitItemRecord> + 'a {
        self.module_specific_trait_items
            .into_iter()
            .map(|arr| arr.iter().copied())
            .flatten()
            .chain(
                self.prelude_trait_items
                    .into_iter()
                    .flat_map(|arr| arr.iter().copied()),
            )
    }
}
