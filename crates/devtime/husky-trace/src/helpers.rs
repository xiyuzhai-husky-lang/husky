use crate::*;
use husky_entity_syn_tree::helpers::paths::module_item_paths;

#[salsa::tracked(jar = TraceJar, return_ref)]
pub(crate) fn root_traces(db: &dyn TraceDb, crate_path: CratePath) -> Vec<Trace> {
    let root_module_path = crate_path.root_module_path(db);
    let Ok(item_paths) = module_item_paths(db, root_module_path) else {
        unreachable!("module path should be guaranteed to be valid")
    };
    item_paths
        .iter()
        .filter_map(|&item_path| Trace::from_item_path(item_path, db))
        .collect()
}
