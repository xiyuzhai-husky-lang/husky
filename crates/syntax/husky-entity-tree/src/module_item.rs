use husky_print_utils::p;
use husky_word::{IdentMap, IdentPairMap, Identifier};
use salsa::DebugWithDb;
use vec_like::{VecMapEntry, VecSet};

use crate::*;
use std::collections::HashMap;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn module_items_map(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> EntityTreeResult<HashMap<EntityPath, ModuleItemMap>> {
    // let ast_sheet = db.ast_sheet(module).as_ref()?;
    Ok(ModuleItemCollector::new(db, crate_path)?.collect_all())
}

#[test]
fn module_items_map_works() {
    DB::expect_test_crates_debug_with_db("module_items_map", |db, crate_path| {
        module_items_map(db, crate_path)
    })
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ModuleItem {
    Defn {
        ident: Identifier,
        accessibility: Accessibility,
        tree_idx: EntityTreeIdx,
    },
    Use {
        ident: Identifier,
        accessibility: Accessibility,
        path: EntityPath,
        use_expr_idx: EntityUseExprIdx,
    },
}

impl DebugWithDb<dyn EntityTreeDb + '_> for ModuleItem {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityTreeDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) struct ModuleItemMap {
    #[return_ref]
    data: IdentMap<ModuleItem>,
}

impl<Db> salsa::DebugWithDb<Db> for ModuleItemMap
where
    Db: EntityTreeDb,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn EntityTreeDb, include_all_fields)
    }
}

impl VecMapEntry<Identifier> for ModuleItem {
    fn key(&self) -> Identifier {
        match self {
            ModuleItem::Defn { ident, .. } | ModuleItem::Use { ident, .. } => *ident,
        }
    }
}

struct ModuleItemCollector<'a> {
    db: &'a dyn EntityTreeDb,
    crate_path: CratePath,
    core_prelude_path: EntityPath,
    prelude: PreludeInAction<'a>,
    root: EntityPath,
    module_item_maps: HashMap<EntityPath, IdentMap<ModuleItem>>,
    use_expr_caches: HashMap<EntityPath, EntityUseExprCache<'a>>,
    errors: Vec<(AstIdx, EntityTreeError)>,
}

pub enum PreludeInAction<'a> {
    Ongoing(IdentMap<ModuleItem>),
    Finished(&'a IdentMap<ModuleItem>),
}

struct EntityUseExprCache<'a> {
    sheet: &'a EntityUseExprSheet,
    unresolved_use_exprs_map: IdentPairMap<EntityUseExprIdx>,
}

impl<'a> ModuleItemCollector<'a> {
    fn new(db: &'a dyn EntityTreeDb, crate_path: CratePath) -> EntityTreeResult<Self> {
        let all_modules = all_modules_within_crate(db, crate_path).as_ref()?;
        let toolchain = db.crate_toolchain(crate_path).as_ref()?;
        let entity_path_menu = db.entity_path_menu(*toolchain);
        let prelude = match crate_prelude(db, crate_path).as_ref()?.as_ref() {
            Some(map) => PreludeInAction::Finished(map.data(db)),
            None => PreludeInAction::Ongoing(Default::default()),
        };
        Ok(Self {
            db,
            crate_path,
            core_prelude_path: entity_path_menu.core_prelude(),
            prelude,
            root: db.it_entity_path(EntityPathData::CrateRoot(crate_path)),
            module_item_maps: todo!(),
            use_expr_caches: todo!(),
            errors: vec![],
        })
    }

    fn collect_all(&mut self) -> HashMap<EntityPath, ModuleItemMap> {
        todo!();
        self.module_item_maps
            .into_iter()
            .map(|(entity_path, map)| (entity_path, ModuleItemMap::new(self.db, map)))
            .collect()
    }
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn crate_prelude(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> EntityTreeResult<Option<ModuleItemMap>> {
    let toolchain = db.crate_toolchain(crate_path).as_ref()?;
    let package_path_menu = db.package_path_menu(*toolchain);
    let core_library = package_path_menu.core_library();
    if crate_path == core_library {
        Ok(None)
    } else {
        let entity_path_menu = db.entity_path_menu(*toolchain);
        Ok(Some(
            module_items_map(db, core_library).as_ref()?[&entity_path_menu.core_prelude()],
        ))
    }
}
