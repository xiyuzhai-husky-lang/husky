use crate::{
    context::{IsGraphRecursionContext, IsGraphRecursionScheme},
    cycle_group::{CycleGroup, CycleGroupMap},
};
use propagate::{IsGraph, Propagate, PropagationResult};

pub(crate) fn calc_cycle_group_final_values<'db, C: IsGraphRecursionContext<'db>>(
    ctx: C,
    cycle_group: &'db CycleGroup<C::Scheme>,
) -> PropagationResult<CycleGroupMap<C::Scheme>>
where
    [(); <C::Scheme as IsGraphRecursionScheme>::CYCLE_GROUP_N]:,
{
    if cycle_group.len() == 1 {
        let node = cycle_group[0];
        let initial_value = ctx.initial_value(node);
        return Ok(CycleGroupMap::new_one_element_map(
            node,
            ctx.updated_value(node, |dep_node| {
                if dep_node == node {
                    &initial_value
                } else {
                    ctx.value(dep_node).expect("todo: handle error")
                }
            }),
        ));
    }
    let local_graph = LocalGraph::new(ctx, cycle_group);
    Ok(local_graph
        .propagate(<C::Scheme as IsGraphRecursionScheme>::MAX_ITERATION)?
        .finish())
}

struct LocalGraph<'db, C: IsGraphRecursionContext<'db>>
where
    [(); <C::Scheme>::CYCLE_GROUP_N]:,
{
    ctx: C,
    map: CycleGroupMap<C::Scheme>,
    deps: Vec<Vec<usize>>,
}

impl<'db, C: IsGraphRecursionContext<'db>> std::fmt::Debug for LocalGraph<'db, C>
where
    [(); <C::Scheme>::CYCLE_GROUP_N]:,
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

impl<'db, C: IsGraphRecursionContext<'db>> LocalGraph<'db, C>
where
    [(); <C::Scheme>::CYCLE_GROUP_N]:,
{
    fn new(ctx: C, cycle_group: &'db CycleGroup<C::Scheme>) -> Self {
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

impl<'db, C: IsGraphRecursionContext<'db>> LocalGraph<'db, C>
where
    [(); <C::Scheme>::CYCLE_GROUP_N]:,
{
    fn finish(self) -> CycleGroupMap<C::Scheme> {
        self.map
    }
}

impl<'db, C: IsGraphRecursionContext<'db>> IsGraph for LocalGraph<'db, C>
where
    [(); <C::Scheme>::CYCLE_GROUP_N]:,
{
    type Value = <C::Scheme as IsGraphRecursionScheme>::Value;

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
