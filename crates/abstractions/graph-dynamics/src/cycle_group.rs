use crate::context::{IsGraphRecursionContext, IsGraphRecursionScheme};
use vec_like::{ordered_small_vec_map::OrderedSmallVecMap, OrderedSmallVecSet};

pub struct CycleGroup<S: IsGraphRecursionScheme>
where
    [(); S::CYCLE_GROUP_N]:,
{
    nodes: OrderedSmallVecSet<S::Node, { S::CYCLE_GROUP_N }>,
}

impl<S: IsGraphRecursionScheme> std::fmt::Debug for CycleGroup<S>
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

impl<S: IsGraphRecursionScheme> PartialEq for CycleGroup<S>
where
    [(); S::CYCLE_GROUP_N]:,
    S::Node: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.nodes == other.nodes
    }
}

impl<S: IsGraphRecursionScheme> Eq for CycleGroup<S>
where
    [(); S::CYCLE_GROUP_N]:,
    S::Node: Eq,
{
}

impl<S: IsGraphRecursionScheme> Clone for CycleGroup<S>
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

impl<S: IsGraphRecursionScheme> std::hash::Hash for CycleGroup<S>
where
    [(); S::CYCLE_GROUP_N]:,
    S::Node: std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.nodes.hash(state);
    }
}

/// # constructor
impl<'a, S: IsGraphRecursionScheme> CycleGroup<S>
where
    [(); S::CYCLE_GROUP_N]:,
{
    pub(crate) fn calc<'db, C: IsGraphRecursionContext<'db, Scheme = S>>(
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

impl<'a, S: IsGraphRecursionScheme> CycleGroup<S>
where
    [(); S::CYCLE_GROUP_N]:,
{
    pub fn nodes(&self) -> &OrderedSmallVecSet<S::Node, { S::CYCLE_GROUP_N }> {
        &self.nodes
    }
}

pub struct CycleGroupMap<S: IsGraphRecursionScheme>
where
    [(); S::CYCLE_GROUP_N]:,
{
    map: OrderedSmallVecMap<(S::Node, S::Value), { S::CYCLE_GROUP_N }>,
}

impl<S: IsGraphRecursionScheme> std::ops::Deref for CycleGroupMap<S>
where
    [(); S::CYCLE_GROUP_N]:,
{
    type Target = OrderedSmallVecMap<(S::Node, S::Value), { S::CYCLE_GROUP_N }>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<S: IsGraphRecursionScheme> std::fmt::Debug for CycleGroupMap<S>
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

impl<S: IsGraphRecursionScheme> PartialEq for CycleGroupMap<S>
where
    [(); S::CYCLE_GROUP_N]:,
    S::Node: PartialEq,
    S::Value: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

impl<S: IsGraphRecursionScheme> Eq for CycleGroupMap<S>
where
    [(); S::CYCLE_GROUP_N]:,
    S::Node: Eq,
    S::Value: Eq,
{
}

impl<S: IsGraphRecursionScheme> CycleGroupMap<S>
where
    [(); S::CYCLE_GROUP_N]:,
{
    pub(crate) fn new<'db, C: IsGraphRecursionContext<'db, Scheme = S>>(
        ctx: C,
        cycle_group: &'db CycleGroup<S>,
    ) -> Self {
        CycleGroupMap {
            map: cycle_group
                .nodes
                .map_collect(|node| ctx.initial_value(node)),
        }
    }
}

impl<S: IsGraphRecursionScheme> std::ops::Index<S::Node> for CycleGroupMap<S>
where
    [(); S::CYCLE_GROUP_N]:,
{
    type Output = S::Value;

    fn index(&self, index: S::Node) -> &Self::Output {
        &self.map[index].1
    }
}

impl<S: IsGraphRecursionScheme> std::ops::IndexMut<S::Node> for CycleGroupMap<S>
where
    [(); S::CYCLE_GROUP_N]:,
{
    #[track_caller]
    fn index_mut(&mut self, index: S::Node) -> &mut Self::Output {
        unsafe { &mut self.map.get_entry_mut(index).expect("index out of bound").1 }
    }
}
