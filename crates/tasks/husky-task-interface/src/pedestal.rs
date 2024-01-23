use serde::{Deserialize, Serialize};

pub trait IsPedestal:
    std::fmt::Debug
    + Default
    + PartialEq
    + Eq
    + Clone
    + Copy
    + Send
    + Sync
    + Serialize
    + std::hash::Hash
    + for<'a> Deserialize<'a>
    + 'static
{
    type UiBuffer: IsPedestalUiBuffer<Pedestal = Self>;

    fn init_ui_buffer(self) -> Self::UiBuffer;
}

pub trait IsPedestalUiBuffer {
    type Pedestal;

    fn update(&mut self, pedestal: Self::Pedestal);
}

impl IsPedestal for () {
    type UiBuffer = ();

    fn init_ui_buffer(self) -> () {
        ()
    }
}

impl IsPedestalUiBuffer for () {
    type Pedestal = ();

    fn update(&mut self, pedestal: Self::Pedestal) {}
}
