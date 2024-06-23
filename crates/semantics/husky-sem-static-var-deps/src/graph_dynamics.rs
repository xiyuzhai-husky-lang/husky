mod initial_value;

use crate::{builder::SemStaticVarDepsBuilder, static_var_deps::SemStaticVarDeps};
use ::graph_dynamics::context::{IsGraphDynamicsContext, IsGraphDynamicsScheme};
use husky_entity_path::{path::ItemPath, region::RegionPath};
use husky_sem_item_path_deps::{
    helpers::graph_dynamics::{
        item_sem_item_path_cycle_group_itd, item_sem_item_path_full_deps_cropped,
        SemItemPathDepsCyclceGroupItd, SemItemPathDepsGraphDepsScheme,
    },
    item_path_deps::item_sem_item_path_deps,
};
use propagate::PropagationResultRef;

pub struct SemStaticVarDepsGraphDynamicsScheme {}

impl IsGraphDynamicsScheme for SemStaticVarDepsGraphDynamicsScheme {
    type Value = SemStaticVarDeps;

    const MAX_ITERATION: usize = 1000;
}

#[derive(Clone, Copy)]
pub struct SemStaticVarDepsGraphDynamicsContext<'db> {
    db: &'db ::salsa::Db,
}

impl<'db> IsGraphDynamicsContext<'db> for SemStaticVarDepsGraphDynamicsContext<'db> {
    type DepsScheme = SemItemPathDepsGraphDepsScheme;

    type DynamicsScheme = SemStaticVarDepsGraphDynamicsScheme;

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

    fn initial_value(self, node: ItemPath) -> SemStaticVarDeps {
        todo!("consider attrs")
    }

    fn updated_value<'a>(
        self,
        node: ItemPath,
        f: impl Fn(ItemPath) -> &'a SemStaticVarDeps,
    ) -> SemStaticVarDeps {
        let mut prev_value = f(node).clone();
        let Some(builder) = SemStaticVarDepsBuilder::new(self.db, RegionPath::ItemDefn(node), f)
        else {
            return prev_value;
        };
        todo!()
    }

    fn cycle_group_final_values(
        self,
        cycle_group_itd: SemItemPathDepsCyclceGroupItd,
    ) -> PropagationResultRef<
        'db,
        &'db graph_dynamics::cycle_group::CycleGroupMap<Self::DepsScheme, SemStaticVarDeps>,
    > {
        todo!()
    }
}
