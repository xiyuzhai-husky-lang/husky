mod action;

use crate::*;
use husky_print_utils::p;
use vec_like::{AsVecMapEntry, VecMap};

pub(crate) struct EntitySymbolCollector<'a> {
    db: &'a dyn EntityTreeDb,
    crate_path: CratePath,
    core_prelude_module: Option<ModulePath>,
    presheets: VecMap<EntitySymbolPresheet>,
    crate_specific_prelude: &'a [EntityNode],
}

impl<'a> EntitySymbolCollector<'a> {
    pub(crate) fn new(db: &'a dyn EntityTreeDb, crate_path: CratePath) -> EntityTreeResult<Self> {
        let all_modules = db.all_modules_within_crate(crate_path)?;
        let presheets = all_modules
            .into_iter()
            .map(|module_path| entity_tree_presheet(db, *module_path).clone())
            .collect::<VfsResult<VecMap<EntitySymbolPresheet>>>()?;
        let core_prelude_module = match crate_path.package_path(db).data(db) {
            PackagePathData::Toolchain { ident, toolchain } => todo!(),
            _ => None,
        };
        Ok(Self {
            db,
            crate_path,
            core_prelude_module,
            presheets,
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
