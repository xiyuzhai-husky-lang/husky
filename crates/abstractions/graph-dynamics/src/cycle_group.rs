use crate::{
    context::{IsGraphDynamicsContext, IsGraphDynamicsScheme},
    deps::{IsGraphDepsContext, IsGraphDepsScheme},
};
use vec_like::{ordered_small_vec_map::OrderedSmallVecMap, OrderedSmallVecSet};

pub struct CycleGroup<S: IsGraphDepsScheme>
where
    [(); S::CYCLE_GROUP_N]:,
{
    nodes: OrderedSmallVecSet<S::Node, { S::CYCLE_GROUP_N }>,
}

impl<S: IsGraphDepsScheme> std::ops::Deref for CycleGroup<S>
where
    [(); S::CYCLE_GROUP_N]:,
{
    type Target = OrderedSmallVecSet<S::Node, { S::CYCLE_GROUP_N }>;

    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

impl<S: IsGraphDepsScheme> std::fmt::Debug for CycleGroup<S>
where
    [(); S::CYCLE_GROUP_N]:,
    S::Node: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CycleGroup")
            .field("nodes", &self.nodes)
            .finish()
    }
}

impl<S: IsGraphDepsScheme> PartialEq for CycleGroup<S>
where
    [(); S::CYCLE_GROUP_N]:,
    S::Node: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.nodes == other.nodes
    }
}

impl<S: IsGraphDepsScheme> Eq for CycleGroup<S>
where
    [(); S::CYCLE_GROUP_N]:,
    S::Node: Eq,
{
}

impl<S: IsGraphDepsScheme> Clone for CycleGroup<S>
where
    [(); S::CYCLE_GROUP_N]:,
    S::Node: Clone,
{
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
        }
    }
}

impl<S: IsGraphDepsScheme> std::hash::Hash for CycleGroup<S>
where
    [(); S::CYCLE_GROUP_N]:,
    S::Node: std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.nodes.hash(state);
    }
}

/// # constructor
impl<'a, S: IsGraphDepsScheme> CycleGroup<S>
where
    [(); S::CYCLE_GROUP_N]:,
{
    pub(crate) fn calc<'db, C: IsGraphDepsContext<'db, Scheme = S>>(
        ctx: C,
        node: S::Node,
    ) -> CycleGroup<S> {
        let nodes = ctx
            .full_deps_cropped(node)
            .iter()
            .copied()
            .filter(|&dep_node| ctx.full_deps_cropped(dep_node).contains(&node))
            .collect();
        CycleGroup { nodes }
    }
}

/// # getters

impl<'a, S: IsGraphDepsScheme> CycleGroup<S>
where
    [(); S::CYCLE_GROUP_N]:,
{
    pub fn nodes(&self) -> &OrderedSmallVecSet<S::Node, { S::CYCLE_GROUP_N }> {
        &self.nodes
    }
}

pub struct CycleGroupMap<S: IsGraphDynamicsScheme>
where
    [(); S::CYCLE_GROUP_N]:,
{
    map: OrderedSmallVecMap<(S::Node, S::Value), { S::CYCLE_GROUP_N }>,
}

impl<S: IsGraphDynamicsScheme> std::ops::Deref for CycleGroupMap<S>
where
    [(); S::CYCLE_GROUP_N]:,
{
    type Target = OrderedSmallVecMap<(S::Node, S::Value), { S::CYCLE_GROUP_N }>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<S: IsGraphDynamicsScheme> std::fmt::Debug for CycleGroupMap<S>
where
    [(); S::CYCLE_GROUP_N]:,
    S::Node: std::fmt::Debug,
    S::Value: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CycleGroupMap")
            .field("nodes", &self.map)
            .finish()
    }
}

impl<S: IsGraphDynamicsScheme> PartialEq for CycleGroupMap<S>
where
    [(); S::CYCLE_GROUP_N]:,
    S::Node: PartialEq,
    S::Value: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

impl<S: IsGraphDynamicsScheme> Eq for CycleGroupMap<S>
where
    [(); S::CYCLE_GROUP_N]:,
    S::Node: Eq,
    S::Value: Eq,
{
}

impl<S: IsGraphDynamicsScheme> CycleGroupMap<S>
where
    [(); S::CYCLE_GROUP_N]:,
{
    pub(crate) fn new<'db, C: IsGraphDynamicsContext<'db, Scheme = S>>(
        ctx: C,
        cycle_group: &'db CycleGroup<S>,
    ) -> Self {
        Self {
            map: cycle_group
                .nodes
                .map_collect(|node| ctx.initial_value(node)),
        }
    }
    pub(crate) fn new_one_element_map(node: S::Node, value: S::Value) -> Self {
        Self {
            map: OrderedSmallVecMap::new_one_element_map((node, value)),
        }
    }
}

impl<S: IsGraphDynamicsScheme> CycleGroupMap<S>
where
    [(); S::CYCLE_GROUP_N]:,
{
    pub(crate) unsafe fn entries_mut(&mut self) -> &mut [(S::Node, S::Value)] {
        self.map.entries_mut()
    }
}

impl<S: IsGraphDynamicsScheme> std::ops::Index<S::Node> for CycleGroupMap<S>
where
    [(); S::CYCLE_GROUP_N]:,
{
    type Output = S::Value;

    fn index(&self, index: S::Node) -> &Self::Output {
        &self.map[index].1
    }
}

impl<S: IsGraphDynamicsScheme> std::ops::IndexMut<S::Node> for CycleGroupMap<S>
where
    [(); S::CYCLE_GROUP_N]:,
{
    #[track_caller]
    fn index_mut(&mut self, index: S::Node) -> &mut Self::Output {
        unsafe { &mut self.map.get_entry_mut(index).expect("index out of bound").1 }
    }
}
