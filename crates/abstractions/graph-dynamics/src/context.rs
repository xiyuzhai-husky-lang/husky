use crate::{
    cycle_group::{CycleGroup, CycleGroupMap},
    full_deps_cropped::calc_full_deps_cropped,
};

pub trait IsGraphRecursionScheme: 'static {
    type Node: Eq + Ord + Copy + 'static;
    type Value;
    const CYCLE_GROUP_N: usize;
    type CycleGroupItd: Copy;
}

pub trait IsGraphRecursionContext<'db>: Copy {
    type Scheme: IsGraphRecursionScheme;

    /// crop deps that definitely are not going to form a cycle
    fn deps_cropped(self, node: Node<Self::Scheme>)
        -> impl IntoIterator<Item = Node<Self::Scheme>>;
    /// final
    fn calc_full_deps_cropped(self, node: Node<Self::Scheme>) -> Vec<Node<Self::Scheme>> {
        calc_full_deps_cropped(self, node)
    }
    /// cached version
    fn full_deps_cropped(self, node: Node<Self::Scheme>) -> &'db [Node<Self::Scheme>];

    /// # cycle group
    /// final
    fn calc_cycle_group(self, node: Node<Self::Scheme>) -> CycleGroup<Self::Scheme>
    where
        [(); <Self::Scheme as IsGraphRecursionScheme>::CYCLE_GROUP_N]:,
    {
        CycleGroup::calc(self, node)
    }
    /// cached version
    fn cycle_group_itd(self, node: Node<Self::Scheme>) -> CycleGroupItd<Self::Scheme>
    where
        [(); <Self::Scheme as IsGraphRecursionScheme>::CYCLE_GROUP_N]:;

    fn initial_value(self, node: Node<Self::Scheme>) -> Value<Self::Scheme>;
    fn updated_value(
        self,
        node: Node<Self::Scheme>,
        query: impl Fn(Node<Self::Scheme>) -> Value<Self::Scheme>,
    ) -> Value<Self::Scheme>
    where
        [(); <Self::Scheme as IsGraphRecursionScheme>::CYCLE_GROUP_N]:;
    /// final
    fn calc_cycle_group_final_values(
        self,
        cycle_group: &'db CycleGroup<Self::Scheme>,
    ) -> CycleGroupMap<Self::Scheme>
    where
        [(); <Self::Scheme as IsGraphRecursionScheme>::CYCLE_GROUP_N]:,
    {
        todo!()
    }
    /// cached version
    fn cycle_group_values(
        self,
        cycle_group_itd: CycleGroupItd<Self::Scheme>,
    ) -> &'db CycleGroupMap<Self::Scheme>
    where
        [(); <Self::Scheme as IsGraphRecursionScheme>::CYCLE_GROUP_N]:;
    /// go through interned cycle group
    fn value(self, node: Node<Self::Scheme>) -> &'db Value<Self::Scheme>
    where
        [(); <Self::Scheme as IsGraphRecursionScheme>::CYCLE_GROUP_N]:,
    {
        &self.cycle_group_values(self.cycle_group_itd(node))[node].1
    }
}

type Node<S> = <S as IsGraphRecursionScheme>::Node;
type Value<S> = <S as IsGraphRecursionScheme>::Value;
type CycleGroupItd<S> = <S as IsGraphRecursionScheme>::CycleGroupItd;
