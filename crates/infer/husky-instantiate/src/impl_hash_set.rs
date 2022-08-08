use std::collections::HashSet;

use crate::*;

impl<T> Instantiable for HashSet<T>
where
    T: Instantiable + Clone + Copy + PartialEq + Eq + std::hash::Hash,
    T::Target: Clone + Copy + PartialEq + Eq + std::hash::Hash,
{
    type Target = HashSet<T::Target>;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self::Target {
        self.iter().map(|t| t.instantiate(ctx)).collect()
    }
}
