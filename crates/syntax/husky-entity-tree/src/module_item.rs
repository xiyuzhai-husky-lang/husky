mod exec;
mod prelude;
mod state;

use crate::*;

use husky_word::{IdentMap, Identifier};
use prelude::*;
use salsa::DebugWithDb;
use state::*;

use vec_like::{VecMapEntry, VecPairMap};

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn module_items_map(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> EntityTreeResult<VecPairMap<ModulePath, ModuleItemMap>> {
    Ok(ModuleItemCollector::new(db, crate_path)?.collect_all())
}

struct ModuleItemCollector<'a> {
    db: &'a dyn EntityTreeDb,
    crate_path: CratePath,
    core_prelude_path: ModulePath,
    prelude: Prelude<'a>,
    root: ModulePath,
    state: CollectorState<'a>,
    errors: Vec<(AstIdx, EntityTreeError)>,
}

impl<'a> ModuleItemCollector<'a> {
    fn new(db: &'a dyn EntityTreeDb, crate_path: CratePath) -> EntityTreeResult<Self> {
        let toolchain = crate_path.toolchain(db);
        let path_menu = db.path_menu(toolchain)?;
        let prelude = match crate_prelude(db, crate_path).as_ref()?.as_ref() {
            Some(map) => Prelude::Finished(map.data(db)),
            None => Prelude::Ongoing(Default::default()),
        };
        let root = ModulePath::new_root(db, crate_path);
        Ok(Self {
            db,
            crate_path,
            core_prelude_path: path_menu.core_prelude(),
            prelude,
            root,
            state: CollectorState::new(db, crate_path)?,
            errors: vec![],
        })
    }

    fn collect_all(mut self) -> VecPairMap<ModulePath, ModuleItemMap> {
        self.repeat_exec_all_util_stable();
        self.state.finish(self.db)
    }
}

#[test]
fn module_items_map_works() {
    DB::expect_test_crates_debug_with_db(
        "module_items_map",
        |db, crate_path| -> EntityTreeResult<&VecPairMap<ModulePath, ModuleItemMap>> {
            Ok(module_items_map(db, crate_path).as_ref()?)
        },
    )
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
        use_expr_idx: UseExprIdx,
    },
}

impl DebugWithDb<dyn EntityTreeDb + '_> for ModuleItem {
    fn fmt(
        &self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &dyn EntityTreeDb,
        _include_all_fields: bool,
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

    fn key_ref(&self) -> &Identifier {
        match self {
            ModuleItem::Defn { ident, .. } | ModuleItem::Use { ident, .. } => ident,
        }
    }
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn crate_prelude(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> EntityTreeResult<Option<ModuleItemMap>> {
    let toolchain = crate_path.toolchain(db);
    let path_menu = db.path_menu(toolchain)?;
    let core_library = path_menu.core_library();
    if crate_path == core_library {
        Ok(None)
    } else {
        let entity_path_menu = db.entity_path_menu(toolchain)?;
        Ok(Some(
            module_items_map(db, core_library).as_ref()?[path_menu.core_prelude()].1,
        ))
    }
}
