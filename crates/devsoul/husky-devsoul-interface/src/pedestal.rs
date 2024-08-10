pub mod virtual_pedestal;

use crate::static_var::IsStaticVar;
use crate::ItemPathIdInterface;
use serde::{Deserialize, Serialize};

pub trait IsPedestal:
    std::fmt::Debug
    + Default
    + PartialEq
    + Eq
    + Clone
    + Send
    + Sync
    + std::hash::Hash
    + 'static
    + FromIterator<(ItemPathIdInterface, Self::StaticVarId)>
{
    type StaticVarId: std::fmt::Debug + Copy + Eq;

    fn exclude<V: IsStaticVar<Self::StaticVarId>>(self) -> Self;

    /// a closed point in algebraic geometry is a minimal prime point locally
    fn is_closed(&self, var_deps: &[ItemPathIdInterface]) -> bool;
}

pub trait IsPedestalFull: IsPedestal + Serialize + for<'a> Deserialize<'a> {}

impl<T> IsPedestalFull for T where T: IsPedestal + Serialize + for<'a> Deserialize<'a> {}
