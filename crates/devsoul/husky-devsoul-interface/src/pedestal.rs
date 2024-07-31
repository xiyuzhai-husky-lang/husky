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
    type StaticVarId: Clone + Copy;
    type UiBuffer: IsPedestalUiBuffer<Pedestal = Self>;

    fn exclude<V: IsStaticVar<Self::StaticVarId>>(self) -> Self;

    fn init_ui_buffer(&self) -> Self::UiBuffer;

    /// a closed point in algebraic geometry is a minimal prime point locally
    fn is_closed(&self, var_deps: &[ItemPathIdInterface]) -> bool;
}

pub trait IsPedestalFull: IsPedestal + Serialize + for<'a> Deserialize<'a> {}

impl<T> IsPedestalFull for T where T: IsPedestal + Serialize + for<'a> Deserialize<'a> {}

pub trait IsPedestalUiBuffer {
    type Pedestal;

    fn update(&mut self, pedestal: &Self::Pedestal);
}
