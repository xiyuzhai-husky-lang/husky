use crate::{builder::SemStaticMutDepsBuilder, static_mut_deps::SemStaticMutDeps, *};
use ::graph_dynamics::{
    context::{IsGraphDynamicsContext, IsGraphDynamicsScheme},
    cycle_group::CycleGroupMap,
};
use husky_entity_path::{
    path::{ItemPath, ItemPathId},
    region::RegionPath,
};
use husky_eth_signature::signature::attr::AttrEthTemplate;
use husky_sem_item_path_deps::{
    helpers::graph_dynamics::{
        item_sem_item_path_cycle_group_itd, item_sem_item_path_full_deps_cropped,
        SemItemPathDepsCyclceGroupItd, SemItemPathDepsGraphDepsScheme,
    },
    item_path_deps::item_sem_item_path_deps,
};
use propagate::{PropagationResult, PropagationResultRef};

pub struct SemStaticMutDepsGraphDynamicsScheme {}

impl IsGraphDynamicsScheme for SemStaticMutDepsGraphDynamicsScheme {
    type Value = SemStaticMutDeps;

    const MAX_ITERATION: usize = 1000;
}

#[derive(Clone, Copy)]
pub struct SemStaticMutDepsGraphDynamicsContext<'db> {
    db: &'db ::salsa::Db,
}

impl<'db> IsGraphDynamicsContext<'db> for SemStaticMutDepsGraphDynamicsContext<'db> {
    type DepsScheme = SemItemPathDepsGraphDepsScheme;

    type DynamicsScheme = SemStaticMutDepsGraphDynamicsScheme;

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

    fn cycle_group_itd(self, node: ItemPath) -> SemItemPathDepsCyclceGroupItd {
        item_sem_item_path_cycle_group_itd(self.db, *node)
    }

    fn initial_value(self, node: ItemPath) -> SemStaticMutDeps {
        item_sem_static_mut_deps_cycle_group_initial_value(self.db, *node).clone()
    }

    fn updated_value<'a>(
        self,
        node: ItemPath,
        f: impl Fn(ItemPath) -> &'a SemStaticMutDeps,
    ) -> SemStaticMutDeps {
        let mut value = f(node).clone();
        let Some(mut builder) =
            SemStaticMutDepsBuilder::new(self.db, RegionPath::ItemDefn(node), f)
        else {
            return value;
        };
        value.merge(&builder.calc_root());
        value
    }

    fn cycle_group_final_values(
        self,
        cycle_group_itd: SemItemPathDepsCyclceGroupItd,
    ) -> PropagationResultRef<'db, &'db CycleGroupMap<Self::DepsScheme, SemStaticMutDeps>> {
        item_sem_static_mut_deps_cycle_group_final_values(self.db, cycle_group_itd).as_ref()
    }
}

#[salsa::tracked(return_ref)]
fn item_sem_static_mut_deps_cycle_group_initial_value(
    db: &::salsa::Db,
    item_path_id: ItemPathId,
) -> SemStaticMutDeps {
    use husky_entity_tree::node::attr::HasAttrPaths;

    let attr_paths = item_path_id.attr_paths(db);
    let mut deps = SemStaticMutDeps::default();
    for attr_path in attr_paths {
        use husky_eth_signature::signature::HasEthTemplate;

        let AttrEthTemplate::Deps(deps_eth_template) = attr_path.eth_template(db).unwrap() else {
            continue;
        };
        todo!()
    }
    deps
}

#[salsa::tracked(return_ref)]
fn item_sem_static_mut_deps_cycle_group_final_values(
    db: &::salsa::Db,
    cycle_group_itd: SemItemPathDepsCyclceGroupItd,
) -> PropagationResult<CycleGroupMap<SemItemPathDepsGraphDepsScheme, SemStaticMutDeps>> {
    let ctx = SemStaticMutDepsGraphDynamicsContext { db };
    let cycle_group = cycle_group_itd.cycle_group(db);
    ctx.calc_cycle_group_final_values(cycle_group)
}

pub fn item_sem_static_mut_deps<'db>(
    db: &'db ::salsa::Db,
    item_path_id: ItemPathId,
) -> &'db SemStaticMutDeps {
    let ctx = SemStaticMutDepsGraphDynamicsContext { db };
    ctx.final_value(item_path_id.item_path(db)).unwrap()
}

#[test]
fn item_sem_static_mut_deps_works() {
    use husky_entity_tree::node::ItemSynNodePath;

    DB::ast_rich_test_debug_with_db(
        |db, item_syn_node_path: ItemSynNodePath| {
            item_syn_node_path
                .unambiguous_item_path(db)
                .map(|item_path| item_sem_static_mut_deps(db, *item_path))
        },
        &AstTestConfig::new(
            "item_sem_static_mut_deps",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    );
}
