use serde::{Deserialize, Serialize};

pub trait IsPedestal:
    std::fmt::Debug + Default + PartialEq + Eq + Clone + Send + Sync + std::hash::Hash + 'static
{
    type StaticVarId;
    type UiBuffer: IsPedestalUiBuffer<Pedestal = Self>;

    fn init_ui_buffer(&self) -> Self::UiBuffer;

    /// a closed point in algebraic geometry is a minimal prime point locally
    #[deprecated]
    fn is_closed(&self) -> bool;
}

pub trait IsPedestalFull: IsPedestal + Serialize + for<'a> Deserialize<'a> {}

impl<T> IsPedestalFull for T where T: IsPedestal + Serialize + for<'a> Deserialize<'a> {}

pub trait IsPedestalUiBuffer {
    type Pedestal;

    fn update(&mut self, pedestal: &Self::Pedestal);
}

impl IsPedestal for () {
    type StaticVarId = ();

    type UiBuffer = ();

    fn init_ui_buffer(&self) -> () {
        ()
    }

    fn is_closed(&self) -> bool {
        true
    }
}

impl IsPedestalUiBuffer for () {
    type Pedestal = ();

    fn update(&mut self, _pedestal: &Self::Pedestal) {}
}
