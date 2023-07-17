use husky_print_utils::p;
use vec_like::VecMapGetEntry;

use super::*;

pub struct TraitsInUse {
    prelude_traits: TraitOrderedSet,
    module_specific_traits: TraitOrderedSet,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct TraitInUseItemsTable<'a> {
    prelude_trait_items_table: &'a [(Ident, SmallVec<[TraitInUseItemRecord; 2]>)],
    module_specific_trait_items_table: &'a [(Ident, SmallVec<[TraitInUseItemRecord; 2]>)],
}

impl<'a> TraitInUseItemsTable<'a> {
    pub fn query(
        db: &'a dyn EntityTreeDb,
        module_path: ModulePath,
    ) -> EntityTreeResultRef<'a, Self> {
        let toolchain = module_path.toolchain(db);
        Ok(Self {
            prelude_trait_items_table: if module_path.crate_path(db)
                == db.vfs_path_menu(toolchain).core_library()
            {
                &[]
            } else {
                non_core_crate_prelude_trait_items_table(db, toolchain).as_ref()?
            },
            module_specific_trait_items_table: module_specific_trait_items_table(db, module_path),
        })
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn non_core_crate_prelude_trait_items_table(
    db: &dyn EntityTreeDb,
    toolchain: Toolchain,
) -> EntityTreeResult<TraitInUseItemsTableImpl> {
    Ok(trait_items_table_impl(
        db,
        none_core_crate_universal_prelude(db, toolchain)
            .as_ref()?
            .as_ref(),
    ))
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn module_specific_trait_items_table(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
) -> TraitInUseItemsTableImpl {
    trait_items_table_impl(db, todo!())
}

type TraitInUseItemsTableImpl = SmallVecPairMap<Ident, SmallVec<[TraitInUseItemRecord; 2]>, 16>;

fn trait_items_table_impl(
    db: &dyn EntityTreeDb,
    entity_symbol_table_ref: EntitySymbolTableRef,
) -> TraitInUseItemsTableImpl {
    let mut table: SmallVecPairMap<Ident, SmallVec<[TraitInUseItemRecord; 2]>, 16> =
        Default::default();
    for entry in entity_symbol_table_ref.data().iter() {
        let PrincipalEntityPath::ModuleItem(ModuleItemPath::Trait(trai_path)) = entry.symbol().path(db) else {
            continue
        };
        p!(trai_path.debug(db));
        todo!()
    }
    table
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct TraitInUseItemRecord {
    path: TraitItemPath,
    scope: Scope,
}

impl TraitInUseItemRecord {
    pub fn path(&self) -> TraitItemPath {
        self.path
    }
}

// section: query

/// available trait items with given identifier
/// designed for
#[derive(Debug, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct TraitInUseItemsWithGivenIdent<'a> {
    prelude_trait_items: Option<&'a [TraitInUseItemRecord]>,
    module_specific_trait_items: Option<&'a [TraitInUseItemRecord]>,
}

impl<'a> TraitInUseItemsTable<'a> {
    fn available_trait_items_with_given_ident(
        self,
        ident: Ident,
    ) -> TraitInUseItemsWithGivenIdent<'a> {
        TraitInUseItemsWithGivenIdent {
            prelude_trait_items: self
                .prelude_trait_items_table
                .get_entry(ident)
                .map(|(_, records)| records as &[_]),
            module_specific_trait_items: self
                .module_specific_trait_items_table
                .get_entry(ident)
                .map(|(_, records)| records as &[_]),
        }
    }
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

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn trai_item_table(db: &dyn EntityTreeDb, traits: TraitOrderedSet) -> TraitInUseItemsTableImpl {
    todo!()
}
