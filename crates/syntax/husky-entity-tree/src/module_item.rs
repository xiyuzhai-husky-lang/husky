use husky_word::{IdentMap, IdentPairMap, Identifier};
use vec_like::{VecMapEntry, VecSet};

use crate::*;
use std::collections::HashMap;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn module_items_map(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> EntityTreeResult<HashMap<EntityPath, IdentMap<ModuleItem>>> {
    // let ast_sheet = db.ast_sheet(module).as_ref()?;
    Ok(ModuleItemCollector::new(db, crate_path)?.collect_all())
}

#[test]
fn module_items_map_works() {
    DB::expect_test_crates("module_items_map", |db, crate_path| {
        module_items_map(db, crate_path)
    })
}

#[derive(Debug, PartialEq, Eq)]
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
    prelude: IdentPairMap<EntityPath>,
    root: EntityPath,
    module_item_maps: HashMap<EntityPath, IdentMap<ModuleItem>>,
    use_expr_caches: HashMap<EntityPath, EntityUseExprCache<'a>>,
    errors: Vec<(AstIdx, EntityTreeError)>,
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
        Ok(Self {
            db,
            crate_path,
            core_prelude_path: entity_path_menu.core_prelude(),
            prelude: todo!(),
            root: db.it_entity_path(EntityPathData::CrateRoot(crate_path)),
            module_item_maps: todo!(),
            use_expr_caches: todo!(),
            errors: vec![],
        })
    }

    fn collect_all(&mut self) -> HashMap<EntityPath, IdentMap<ModuleItem>> {
        todo!();
        self.module_item_maps
    }
}
