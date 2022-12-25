mod action;

use crate::*;
use husky_print_utils::p;
use vec_like::{AsVecMapEntry, VecMap};

pub(crate) struct EntityTreeCollector<'a> {
    db: &'a dyn EntityTreeDb,
    crate_path: CratePath,
    presheets: VecMap<EntityTreePresheet>,
    core_prelude_module: ModulePath,
    // can't use `crate_prelude` here because it might not be available
    opt_universal_prelude: Option<&'a [EntitySymbol]>,
    crate_specific_prelude: &'a [EntitySymbol],
}

impl<'a> EntityTreeCollector<'a> {
    pub(crate) fn new(db: &'a dyn EntityTreeDb, crate_path: CratePath) -> EntityTreeResult<Self> {
        let all_modules = db.all_modules_within_crate(crate_path)?;
        let presheets = all_modules
            .into_iter()
            .map(|module_path| entity_tree_presheet(db, *module_path).clone())
            .collect::<VfsResult<VecMap<EntityTreePresheet>>>()?;
        let toolchain = crate_path.toolchain(db);
        let path_menu = db.dev_path_menu()?;
        let core_prelude_module = path_menu.core_prelude();
        let universal_prelude: Option<&'a [EntitySymbol]> = {
            if crate_path != path_menu.core_library() {
                Some(entity_tree_sheet(db, core_prelude_module)?.module_symbols())
            } else {
                None
            }
        };
        Ok(Self {
            db,
            crate_path,
            presheets,
            core_prelude_module,
            opt_universal_prelude: universal_prelude,
            crate_specific_prelude: crate_specific_prelude(db, crate_path).as_ref()?,
        })
    }

    pub(crate) fn collect_all(mut self) -> EntityTreeBundle {
        loop {
            let actions = self.collect_possible_actions();
            if actions.len() == 0 {
                break;
            }
            for action in actions {
                self.exec(action)
            }
        }
        self.collect_associated_items();
        EntityTreeBundle::new(
            self.presheets
                .into_iter()
                .map(|presheet| presheet.into())
                .collect(),
        )
    }

    fn collect_associated_items(&mut self) {
        // todo: collect associated_items
    }

    fn exec(&mut self, action: PresheetAction) {
        let module_path = action.module_path();
        let presheet = &mut self.presheets[module_path];
        presheet.exec(self.db, action)
    }
}
