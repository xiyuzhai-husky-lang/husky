use std::collections::HashSet;
use vec_like::VecSet;

use crate::*;

impl<T> Instantiable for VecSet<T>
where
    T: Instantiable + Clone + Copy + PartialEq + Eq + std::hash::Hash,
    T::Target: Clone + Copy + PartialEq + Eq + std::hash::Hash,
{
    type Target = VecSet<T::Target>;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self::Target {
        self.iter().map(|t| t.instantiate(ctx)).collect()
    }
}
