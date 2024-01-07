use super::*;
use vec_like::VecMapGetEntry;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct TraitInUseItemsTable<'a> {
    prelude_trait_items_table: &'a [(Ident, SmallVec<[TraitInUseItemRecord; 2]>)],
    module_specific_trait_items_table: &'a [(Ident, SmallVec<[TraitInUseItemRecord; 2]>)],
}

impl<'a> TraitInUseItemsTable<'a> {
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
    ) -> Option<TraitInUseItemsWithGivenIdent<'a>> {
        let items = TraitInUseItemsWithGivenIdent {
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

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
fn non_core_crate_prelude_trait_items_table(
    db: &::salsa::Db,
    toolchain: Toolchain,
) -> TraitInUseItemsTableImpl {
    trait_items_table_impl(
        db,
        none_core_crate_universal_prelude(db, toolchain).as_ref(),
    )
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
fn module_specific_trait_items_table(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> TraitInUseItemsTableImpl {
    trait_items_table_impl(db, module_path.item_tree_sheet(db).module_symbols())
}

type TraitInUseItemsTableImpl = SmallVecPairMap<Ident, SmallVec<[TraitInUseItemRecord; 2]>, 16>;

fn trait_items_table_impl(
    db: &::salsa::Db,
    item_symbol_table_ref: EntitySymbolTableRef,
) -> TraitInUseItemsTableImpl {
    let mut table: SmallVecPairMap<Ident, SmallVec<[TraitInUseItemRecord; 2]>, 16> =
        Default::default();
    for entry in item_symbol_table_ref.data().iter() {
        let PrincipalEntityPath::MajorItem(MajorItemPath::Trait(trai_path)) =
            entry.symbol().principal_entity_path(db)
        else {
            continue;
        };
        for (ident, trai_item_path) in trai_path.associated_item_paths(db) {
            let record = TraitInUseItemRecord {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct TraitInUseItemRecord {
    trai_symbol: EntitySymbol,
    trai_path: TraitPath,
    trai_item_path: TraitItemPath,
    scope: Scope,
}

impl TraitInUseItemRecord {
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
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct TraitInUseItemsWithGivenIdent<'a> {
    prelude_trait_items: Option<&'a [TraitInUseItemRecord]>,
    module_specific_trait_items: Option<&'a [TraitInUseItemRecord]>,
}

impl<'a> TraitInUseItemsWithGivenIdent<'a> {
    pub fn records(self) -> impl Iterator<Item = TraitInUseItemRecord> + 'a {
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

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
fn trai_item_table(_db: &::salsa::Db, _traits: TraitOrderedSet) -> TraitInUseItemsTableImpl {
    todo!()
}
