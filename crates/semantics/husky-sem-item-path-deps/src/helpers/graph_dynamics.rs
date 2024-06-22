use super::*;
use crate::item_path_deps::item_sem_item_path_deps;
use ::graph_dynamics::{
    cycle_group::CycleGroup,
    deps::{IsGraphDepsContext, IsGraphDepsScheme},
};
use husky_entity_path::path::{ItemPath, ItemPathId};

#[salsa::interned(constructor = new)]
pub struct SemItemPathCyclceGroupItd {
    pub cycle_group: CycleGroup<SemItemPathGraphDepsScheme>,
}

pub struct SemItemPathGraphDepsScheme;

impl IsGraphDepsScheme for SemItemPathGraphDepsScheme {
    type Node = ItemPath;

    const CYCLE_GROUP_N: usize = 4;

    type CycleGroupItd = SemItemPathCyclceGroupItd;
}

#[derive(Clone, Copy)]
struct SemItemPathGraphDepsContext<'db> {
    db: &'db ::salsa::Db,
}

impl<'db> IsGraphDepsContext<'db> for SemItemPathGraphDepsContext<'db> {
    type Scheme = SemItemPathGraphDepsScheme;

    fn deps_cropped(self, node: ItemPath) -> impl IntoIterator<Item = ItemPath> {
        item_sem_item_path_deps(self.db, *node)
            .as_ref()
            .unwrap()
            .iter()
            .copied()
    }

    fn full_deps_cropped(self, node: ItemPath) -> &'db [ItemPath] {
        item_sem_item_path_full_deps_cropped(self.db, *node)
    }

    fn cycle_group_itd(self, node: ItemPath) -> SemItemPathCyclceGroupItd {
        item_sem_item_path_cycle_group_itd(self.db, *node)
    }
}

#[salsa::tracked(return_ref)]
pub fn item_sem_item_path_full_deps_cropped(db: &::salsa::Db, node: ItemPathId) -> Vec<ItemPath> {
    let ctx = SemItemPathGraphDepsContext { db };
    ctx.calc_full_deps_cropped(node.item_path(db))
}

#[test]
fn item_sem_item_path_full_deps_cropped_works() {
    use husky_entity_tree::node::ItemSynNodePath;

    DB::ast_rich_test_debug_with_db(
        |db, item_syn_node_path: ItemSynNodePath| {
            item_syn_node_path
                .unambiguous_item_path(db)
                .map(|item_path| item_sem_item_path_full_deps_cropped(db, *item_path))
        },
        &AstTestConfig::new(
            "item_sem_item_path_full_deps_cropped",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    );
}

#[salsa::tracked]
pub fn item_sem_item_path_cycle_group_itd(
    db: &::salsa::Db,
    node: ItemPathId,
) -> SemItemPathCyclceGroupItd {
    let ctx = SemItemPathGraphDepsContext { db };
    let cycle_group = ctx.calc_cycle_group(node.item_path(db));
    SemItemPathCyclceGroupItd::new(db, cycle_group)
}

#[test]
fn item_sem_item_path_cycle_group_itd_works() {
    use husky_entity_tree::node::ItemSynNodePath;

    DB::ast_rich_test_debug_with_db(
        |db, item_syn_node_path: ItemSynNodePath| {
            item_syn_node_path
                .unambiguous_item_path(db)
                .map(|item_path| item_sem_item_path_cycle_group_itd(db, *item_path))
        },
        &AstTestConfig::new(
            "item_sem_item_path_cycle_group_itd",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    );
}
