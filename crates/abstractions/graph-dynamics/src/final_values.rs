use crate::{
    context::{IsGraphDynamicsContext, IsGraphDynamicsScheme},
    cycle_group::{CycleGroup, CycleGroupMap},
    deps::IsGraphDepsScheme,
};
use propagate::{IsGraph, Propagate, PropagationResult};

pub(crate) fn calc_cycle_group_final_values<'db, C: IsGraphDynamicsContext<'db>>(
    ctx: C,
    cycle_group: &'db CycleGroup<C::DepsScheme>,
) -> PropagationResult<
    CycleGroupMap<C::DepsScheme, <C::DynamicsScheme as IsGraphDynamicsScheme>::Value>,
>
where
    [(); <C::DepsScheme as IsGraphDepsScheme>::CYCLE_GROUP_N]:,
{
    let local_graph = LocalGraph::new(ctx, cycle_group);
    Ok(local_graph
        .propagate(<C::DepsScheme as IsGraphDynamicsScheme>::MAX_ITERATION)?
        .finish())
}

struct LocalGraph<'db, C: IsGraphDynamicsContext<'db>>
where
    [(); <C::DepsScheme>::CYCLE_GROUP_N]:,
{
    ctx: C,
    map: CycleGroupMap<C::DepsScheme, <C::DynamicsScheme as IsGraphDynamicsScheme>::Value>,
    deps: Vec<Vec<usize>>,
}

impl<'db, C: IsGraphDynamicsContext<'db>> std::fmt::Debug for LocalGraph<'db, C>
where
    [(); <C::DepsScheme>::CYCLE_GROUP_N]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalGraph")
            .field(
                "map",
                &self
                    .map
                    .iter()
                    .map(|(node, value)| value)
                    .collect::<Vec<_>>(),
            )
            .field("deps", &self.deps)
            .finish()
    }
}

impl<'db, C: IsGraphDynamicsContext<'db>> LocalGraph<'db, C>
where
    [(); <C::DepsScheme>::CYCLE_GROUP_N]:,
{
    fn new(ctx: C, cycle_group: &'db CycleGroup<C::DepsScheme>) -> Self {
        Self {
            ctx,
            map: CycleGroupMap::new(ctx, cycle_group),
            deps: cycle_group
                .iter()
                .map(|&node| {
                    ctx.deps_cropped(node)
                        .into_iter()
                        .filter_map(|dep_node| {
                            cycle_group
                                .iter()
                                .position(|&cycle_group_node| cycle_group_node == dep_node)
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

impl<'db, C: IsGraphDynamicsContext<'db>> LocalGraph<'db, C>
where
    [(); <C::DepsScheme>::CYCLE_GROUP_N]:,
{
    fn finish(
        self,
    ) -> CycleGroupMap<C::DepsScheme, <C::DynamicsScheme as IsGraphDynamicsScheme>::Value> {
        self.map
    }
}

impl<'db, C: IsGraphDynamicsContext<'db>> IsGraph for LocalGraph<'db, C>
where
    [(); <C::DepsScheme>::CYCLE_GROUP_N]:,
{
    type Value = <C::DynamicsScheme as IsGraphDynamicsScheme>::Value;

    fn len(&self) -> usize {
        self.map.len()
    }

    fn deps(&self, idx: usize) -> impl IntoIterator<Item = usize> {
        self.deps[idx].iter().copied()
    }

    fn value_mut(&mut self, idx: usize) -> &mut Self::Value {
        unsafe { &mut self.map.entries_mut()[idx].1 }
    }

    fn eval(&self, idx: usize) -> Self::Value {
        self.ctx.updated_value((**self.map)[idx].0, |node| {
            self.map
                .get_value(node)
                .unwrap_or_else(|| self.ctx.value(node).expect("todo: handle error"))
        })
    }
}
