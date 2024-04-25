mod connected;
mod disconnected;

use super::*;
use husky_regional_token::RegionalTokenIdxRange;
use vec_like::VecMapGetEntry;

/// given the ident of the item, what are the traits and trait item paths and maybe their scopes?
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AvailableTraitItemsTable<'a> {
    prelude_trait_items_table: &'a [(Ident, SmallVec<[(TraitPath, TraitItemPath); 2]>)],
    module_specific_connected_trait_items_table:
        &'a [(Ident, SmallVec<[(TraitPath, TraitItemPath); 2]>)],
    module_specific_disconnected_trait_items_table: &'a [(
        Ident,
        SmallVec<[(TraitPath, TraitItemPath, RegionalTokenIdxRange); 2]>,
    )],
}

impl<'a> AvailableTraitItemsTable<'a> {
    #[deprecated(
        note = "placeholder to avoid calculating `module_specific_disconnected_trait_items_table` for each region"
    )]
    pub fn new_ad_hoc(db: &'a ::salsa::Db, module_path: ModulePath) -> Self {
        Self::new(db, module_path, /* ad hoc */ &[])
    }

    pub fn new(
        db: &'a ::salsa::Db,
        module_path: ModulePath,
        module_specific_disconnected_trait_items_table: &'a [(
            Ident,
            SmallVec<[(TraitPath, TraitItemPath, RegionalTokenIdxRange); 2]>,
        )],
    ) -> Self {
        let toolchain = module_path.toolchain(db);
        Self {
            prelude_trait_items_table: if module_path.crate_path(db)
                == db.vfs_path_menu(toolchain).core_library()
            {
                &[]
            } else {
                non_core_crate_prelude_trait_item_records(db, toolchain)
            },
            module_specific_connected_trait_items_table: module_specific_trait_item_records(
                db,
                module_path,
            ),
            module_specific_disconnected_trait_items_table,
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
            module_specific_connected_trait_items: self
                .module_specific_connected_trait_items_table
                .get_entry(ident)
                .map(|(_, records)| records as &[_]),
            module_specific_disconnected_trait_items: self
                .module_specific_disconnected_trait_items_table
                .get_entry(ident)
                .map(|(_, records)| records as &[_]),
        };
        if items.prelude_trait_items.is_none()
            && items.module_specific_connected_trait_items.is_none()
            && items.module_specific_disconnected_trait_items.is_none()
        {
            return None;
        }
        Some(items)
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn non_core_crate_prelude_trait_item_records(
    db: &::salsa::Db,
    toolchain: Toolchain,
) -> AvailableTraitItemRecords {
    trait_item_records(
        db,
        none_core_crate_universal_prelude(db, toolchain).as_ref(),
    )
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn module_specific_trait_item_records(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> AvailableTraitItemRecords {
    trait_item_records(db, module_path.item_tree_sheet(db).module_symbols())
}

type AvailableTraitItemRecords =
    SmallVecPairMap<Ident, SmallVec<[(TraitPath, TraitItemPath); 2]>, 16>;

fn trait_item_records(
    db: &::salsa::Db,
    item_symbol_table_ref: EntitySymbolTableRef,
) -> AvailableTraitItemRecords {
    let mut table: SmallVecPairMap<Ident, SmallVec<[(TraitPath, TraitItemPath); 2]>, 16> =
        Default::default();
    for entry in item_symbol_table_ref.data().iter() {
        let PrincipalEntityPath::MajorItem(MajorItemPath::Trait(trai_path)) =
            entry.symbol().principal_entity_path(db)
        else {
            continue;
        };
        for &(ident, trai_item_path) in trai_path.assoc_item_paths(db) {
            match entry.visible_scope() {
                Scope::Pub | Scope::PubUnder(_) | Scope::Private(_) => (),
                Scope::Disconnected { .. } => continue,
            }
            table.update_value_or_insert(
                ident,
                |records| records.push((trai_path, trai_item_path)),
                smallvec![(trai_path, trai_item_path)],
            )
        }
    }
    table
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AvailableTraitItemRecord {
    trai_path: TraitPath,
    trai_item_path: TraitItemPath,
    visible_scope: Scope,
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
    prelude_trait_items: Option<&'a [(TraitPath, TraitItemPath)]>,
    module_specific_connected_trait_items: Option<&'a [(TraitPath, TraitItemPath)]>,
    module_specific_disconnected_trait_items:
        Option<&'a [(TraitPath, TraitItemPath, RegionalTokenIdxRange)]>,
}

impl<'a> IntoIterator for AvailableTraitItemsWithGivenIdent<'a> {
    type Item = (TraitPath, TraitItemPath, Option<RegionalTokenIdxRange>);

    type IntoIter =
        impl Iterator<Item = (TraitPath, TraitItemPath, Option<RegionalTokenIdxRange>)> + 'a;

    fn into_iter(self) -> Self::IntoIter {
        self.prelude_trait_items
            .into_iter()
            .map(|arr| arr.iter().copied())
            .flatten()
            .chain(
                self.module_specific_connected_trait_items
                    .into_iter()
                    .flat_map(|arr| arr.iter().copied()),
            )
            .map(|(trai_path, trai_item_path)| (trai_path, trai_item_path, None))
            .chain(
                self.module_specific_disconnected_trait_items
                    .into_iter()
                    .flat_map(|arr| {
                        arr.iter().map(|&(trai_path, trai_item_path, range)| {
                            (trai_path, trai_item_path, Some(range))
                        })
                    }),
            )
    }
}
