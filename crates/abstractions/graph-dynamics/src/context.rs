use propagate::{PropagationResult, PropagationResultRef};

use crate::{
    cycle_group::{CycleGroup, CycleGroupMap},
    deps::{calc_full_deps_cropped, IsGraphDepsScheme},
    final_values::calc_cycle_group_final_values,
};

pub trait IsGraphDynamicsScheme: 'static {
    type Value: Eq + std::fmt::Debug;
    const MAX_ITERATION: usize;
}

pub trait IsGraphDynamicsContext<'db>: Copy {
    type DepsScheme: IsGraphDepsScheme;
    type DynamicsScheme: IsGraphDynamicsScheme;
    fn debug_node(self, node: Node<Self::DepsScheme>) -> String;

    /// crop deps that definitely are not going to form a cycle
    fn deps_cropped(
        self,
        node: Node<Self::DepsScheme>,
    ) -> impl IntoIterator<Item = Node<Self::DepsScheme>>;
    /// cached version
    fn full_deps_cropped(self, node: Node<Self::DepsScheme>) -> &'db [Node<Self::DepsScheme>];

    /// cached version
    fn cycle_group_itd(self, node: Node<Self::DepsScheme>) -> CycleGroupItd<Self::DepsScheme>
    where
        [(); <Self::DepsScheme as IsGraphDepsScheme>::CYCLE_GROUP_N]:;
    fn initial_value(self, node: Node<Self::DepsScheme>) -> Value<Self::DynamicsScheme>;
    fn updated_value<'a>(
        self,
        node: Node<Self::DepsScheme>,
        query: impl Fn(Node<Self::DepsScheme>) -> &'a Value<Self::DynamicsScheme>,
    ) -> Value<Self::DynamicsScheme>
    where
        [(); <Self::DepsScheme as IsGraphDepsScheme>::CYCLE_GROUP_N]:;
    /// final
    fn calc_cycle_group_final_values(
        self,
        cycle_group: &'db CycleGroup<Self::DepsScheme>,
    ) -> PropagationResult<CycleGroupMap<Self::DepsScheme, Value<Self::DynamicsScheme>>>
    where
        [(); <Self::DepsScheme as IsGraphDepsScheme>::CYCLE_GROUP_N]:,
    {
        calc_cycle_group_final_values(self, cycle_group)
    }
    /// cached version of `Self::calc_cycle_group_final_values`
    fn cycle_group_final_values(
        self,
        cycle_group_itd: CycleGroupItd<Self::DepsScheme>,
    ) -> PropagationResultRef<'db, &'db CycleGroupMap<Self::DepsScheme, Value<Self::DynamicsScheme>>>
    where
        [(); <Self::DepsScheme as IsGraphDepsScheme>::CYCLE_GROUP_N]:;
    /// obtained through interned cycle group final values
    fn final_value(
        self,
        node: Node<Self::DepsScheme>,
    ) -> PropagationResultRef<'db, &'db Value<Self::DynamicsScheme>>
    where
        [(); <Self::DepsScheme as IsGraphDepsScheme>::CYCLE_GROUP_N]:,
    {
        Ok(&self.cycle_group_final_values(self.cycle_group_itd(node))?[node])
    }
}

type Node<S> = <S as IsGraphDepsScheme>::Node;
type CycleGroupItd<S> = <S as IsGraphDepsScheme>::CycleGroupItd;
type Value<S> = <S as IsGraphDynamicsScheme>::Value;
