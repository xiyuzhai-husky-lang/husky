use crate::{
    cycle_group::{CycleGroup, CycleGroupMap},
    full_deps_cropped::calc_full_deps_cropped,
};

pub trait IsGraphRecursionScheme {
    type Node: Eq + Ord + Copy + 'static;
    type Value;
    const CYCLE_GROUP_N: usize;
    type CycleGroupItd: Copy;
}

pub trait IsGraphRecursionContext<'db>: Copy {
    type Scheme: IsGraphRecursionScheme;

    /// crop deps that definitely are not going to form a cycle
    fn deps_cropped(
        self,
        node: <Self::Scheme as IsGraphRecursionScheme>::Node,
    ) -> &'db [<Self::Scheme as IsGraphRecursionScheme>::Node];
    /// final
    fn calc_full_deps_cropped(
        self,
        node: <Self::Scheme as IsGraphRecursionScheme>::Node,
    ) -> Vec<<Self::Scheme as IsGraphRecursionScheme>::Node> {
        calc_full_deps_cropped(self, node)
    }
    /// cached version
    fn full_deps_cropped(
        self,
        node: <Self::Scheme as IsGraphRecursionScheme>::Node,
    ) -> &'db [<Self::Scheme as IsGraphRecursionScheme>::Node];

    /// # cycle group
    /// final
    fn calc_cycle_group(
        self,
        node: <Self::Scheme as IsGraphRecursionScheme>::Node,
    ) -> CycleGroup<Self::Scheme>
    where
        [(); <Self::Scheme as IsGraphRecursionScheme>::CYCLE_GROUP_N]:,
    {
        CycleGroup::calc(self, node)
    }
    /// cached version
    fn cycle_group(
        self,
        node: <Self::Scheme as IsGraphRecursionScheme>::Node,
    ) -> <Self::Scheme as IsGraphRecursionScheme>::CycleGroupItd
    where
        [(); <Self::Scheme as IsGraphRecursionScheme>::CYCLE_GROUP_N]:;

    fn initial_value(
        self,
        node: <Self::Scheme as IsGraphRecursionScheme>::Node,
    ) -> <Self::Scheme as IsGraphRecursionScheme>::Value;
    fn calc_value_step(
        self,
        node: <Self::Scheme as IsGraphRecursionScheme>::Node,
        cycle_group_map: &'db CycleGroupMap<Self::Scheme>,
    ) -> <Self::Scheme as IsGraphRecursionScheme>::Value
    where
        [(); <Self::Scheme as IsGraphRecursionScheme>::CYCLE_GROUP_N]:;
    /// final
    fn calc_cycle_group_values(
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
        cycle_group_itd: <Self::Scheme as IsGraphRecursionScheme>::CycleGroupItd,
    ) -> &'db CycleGroupMap<Self::Scheme>
    where
        [(); <Self::Scheme as IsGraphRecursionScheme>::CYCLE_GROUP_N]:;
    /// cached version, go through interned cycle group
    fn value(
        self,
        node: <Self::Scheme as IsGraphRecursionScheme>::Node,
    ) -> &'db <Self::Scheme as IsGraphRecursionScheme>::Value;
}
