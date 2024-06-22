use crate::static_mut_deps::SemStaticMutDeps;
use ::graph_dynamics::context::{IsGraphDynamicsContext, IsGraphDynamicsScheme};
use husky_entity_path::path::ItemPath;
use husky_sem_item_path_deps::{
    helpers::graph_dynamics::{SemItemPathDepsCyclceGroupItd, SemItemPathDepsGraphDepsScheme},
    item_path_deps::item_sem_item_path_deps,
};
use propagate::PropagationResultRef;

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
        todo!()
    }

    fn cycle_group_itd(self, node: ItemPath) -> SemItemPathDepsCyclceGroupItd {
        todo!()
    }

    fn initial_value(self, node: ItemPath) -> SemStaticMutDeps {
        todo!("consider attrs")
    }

    fn updated_value<'a>(
        self,
        node: ItemPath,
        query: impl Fn(ItemPath) -> &'a SemStaticMutDeps,
    ) -> SemStaticMutDeps {
        todo!()
    }

    fn cycle_group_values(
        self,
        cycle_group_itd: SemItemPathDepsCyclceGroupItd,
    ) -> PropagationResultRef<
        'db,
        &'db graph_dynamics::cycle_group::CycleGroupMap<Self::DepsScheme, SemStaticMutDeps>,
    > {
        todo!()
    }
}
