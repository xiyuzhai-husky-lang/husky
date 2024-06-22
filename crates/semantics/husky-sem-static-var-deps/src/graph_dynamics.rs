use crate::static_var_deps::SemStaticVarDeps;
use ::graph_dynamics::context::{IsGraphDynamicsContext, IsGraphDynamicsScheme};
use husky_entity_path::path::ItemPath;
use husky_sem_item_path_deps::{
    helpers::graph_dynamics::{SemItemPathDepsCyclceGroupItd, SemItemPathDepsGraphDepsScheme},
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
        todo!()
    }

    fn cycle_group_itd(self, node: ItemPath) -> SemItemPathDepsCyclceGroupItd {
        todo!()
    }

    fn initial_value(self, node: ItemPath) -> SemStaticVarDeps {
        todo!("consider attrs")
    }

    fn updated_value<'a>(
        self,
        node: ItemPath,
        query: impl Fn(ItemPath) -> &'a SemStaticVarDeps,
    ) -> SemStaticVarDeps {
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
