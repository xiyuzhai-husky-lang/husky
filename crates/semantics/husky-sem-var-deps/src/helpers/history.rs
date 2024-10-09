use super::*;
use crate::{region::item_defn_sem_var_deps_region, var_deps::value::SemValueVarDeps};
use husky_entity_path::path::ItemPathId;

#[salsa::tracked]
pub fn item_history_sem_var_deps(
    db: &::salsa::Db,
    item_path_id: ItemPathId,
) -> Option<SemValueVarDeps> {
    item_defn_sem_var_deps_region(db, item_path_id).map(|region| region.total_var_deps(db))
}

#[test]
fn item_history_sem_var_deps_works() {
    DB::ast_rich_test_debug_with_db(
        item_history_sem_var_deps,
        &AstTestConfig::new(
            "item_history_sem_var_deps",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    );
}
