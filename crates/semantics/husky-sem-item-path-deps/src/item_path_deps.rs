use crate::builder::SemItemPathDepsBuilder;
use crate::*;
use husky_entity_path::path::ItemPathId;
use husky_entity_path::{path::ItemPath, region::RegionPath};
use vec_like::VecSet;

#[salsa::tracked(return_ref)]
pub(crate) fn item_item_path_deps(
    db: &::salsa::Db,
    item_path_id: ItemPathId,
) -> SemItemPathDepsResult<VecSet<ItemPath>> {
    let item_path = item_path_id.item_path(db);
    let mut builder = SemItemPathDepsBuilder::new(db);
    builder.add_region(RegionPath::ItemDecl(item_path))?;
    builder.add_region(RegionPath::ItemDefn(item_path))?;
    Ok(builder.finish())
}

#[test]
fn item_item_path_deps_works() {
    use husky_entity_tree::node::ItemSynNodePath;

    DB::ast_rich_test_debug_with_db(
        |db, item_syn_node_path: ItemSynNodePath| {
            item_syn_node_path
                .unambiguous_item_path(db)
                .map(|item_path| item_item_path_deps(db, *item_path))
        },
        &AstTestConfig::new(
            "defn_sem_expr_regions",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    );
}

#[salsa::tracked(return_ref)]
pub(crate) fn item_item_path_deps_cropped(
    db: &::salsa::Db,
    item_path_id: ItemPathId,
) -> SemItemPathDepsResult<VecSet<ItemPath>> {
    todo!()
}
