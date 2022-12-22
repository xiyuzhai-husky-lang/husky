mod action;

use crate::*;
use husky_print_utils::p;
use vec_like::{AsVecMapEntry, VecMap};

pub(crate) struct EntitySymbolCollector<'a> {
    db: &'a dyn EntitySymbolDb,
    presheets: VecMap<&'a EntitySymbolPresheet>,
    crate_specific_prelude: VecMap<EntitySymbol>,
}

impl<'a> EntitySymbolCollector<'a> {
    pub(crate) fn new(db: &'a dyn EntitySymbolDb, crate_path: CratePath) -> VfsResult<Self> {
        let all_modules = db.all_modules_within_crate(crate_path)?;
        let presheets = all_modules
            .into_iter()
            .map(|module_path| Ok(entity_tree_presheet(db, *module_path).as_ref()?))
            .collect::<VfsResult<VecMap<&'a EntitySymbolPresheet>>>()?;
        Ok(Self {
            db,
            presheets,
            crate_specific_prelude: todo!(),
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
