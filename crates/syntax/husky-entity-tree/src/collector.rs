mod action;

use crate::*;
use husky_print_utils::p;
use vec_like::{AsVecMapEntry, VecMap};

pub(crate) struct EntityTreeCollector<'a> {
    db: &'a dyn EntityTreeDb,
    crate_path: CratePath,
    presheets: VecMap<EntitySymbolPresheet>,
    core_prelude_module: ModulePath,
    universal_prelude: Option<&'a [EntityNode]>,
    crate_specific_prelude: &'a [EntityNode],
}

impl<'a> EntityTreeCollector<'a> {
    pub(crate) fn new(db: &'a dyn EntityTreeDb, crate_path: CratePath) -> EntityTreeResult<Self> {
        let all_modules = db.all_modules_within_crate(crate_path)?;
        let presheets = all_modules
            .into_iter()
            .map(|module_path| entity_tree_presheet(db, *module_path).clone())
            .collect::<VfsResult<VecMap<EntitySymbolPresheet>>>()?;
        let toolchain = crate_path.toolchain(db);
        let path_menu = db.path_menu(toolchain)?;
        let core_prelude_module = path_menu.core_prelude();
        let universal_prelude: Option<&'a [EntityNode]> = if crate_path != path_menu.core_library()
        {
            Some(entity_tree_sheet(db, core_prelude_module)?.module_items(db))
        } else {
            None
        };
        Ok(Self {
            db,
            crate_path,
            presheets,
            core_prelude_module,
            universal_prelude,
            crate_specific_prelude: crate_specific_prelude(db, crate_path).as_ref()?,
        })
    }

    pub(crate) fn collect_all(mut self) -> EntitySymbolBundle {
        loop {
            let actions = self.collect_possible_actions();
            if actions.len() == 0 {
                break;
            }
            todo!()
        }
        todo!()
    }
}
