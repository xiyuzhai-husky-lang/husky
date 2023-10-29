use crate::*;
use husky_entity_syn_tree::helpers::paths::module_item_paths;

pub fn module_item_syn_defns(module_path: ModulePath, db: &dyn SynDefnDb) -> Vec<SynDefn> {
    module_item_paths(db, module_path)
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|&item_path| item_path.syn_defn(db).ok())
        .collect()
}
