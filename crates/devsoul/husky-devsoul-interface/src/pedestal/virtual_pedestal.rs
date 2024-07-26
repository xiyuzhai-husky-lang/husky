use super::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VirtualPedestal;

impl FromIterator<(ItemPathIdInterface, ())> for VirtualPedestal {
    fn from_iter<T: IntoIterator<Item = (ItemPathIdInterface, ())>>(iter: T) -> Self {
        Self
    }
}

impl IsPedestal for VirtualPedestal {
    type StaticVarId = ();

    type UiBuffer = VirtualPedestalUiBuffer;

    fn exclude<V: IsStaticVar<()>>(&mut self) {}

    fn init_ui_buffer(&self) -> Self::UiBuffer {
        todo!()
    }

    fn is_closed(&self) -> bool {
        todo!()
    }
}

pub struct VirtualPedestalUiBuffer;

impl IsPedestalUiBuffer for VirtualPedestalUiBuffer {
    type Pedestal = VirtualPedestal;

    fn update(&mut self, pedestal: &Self::Pedestal) {
        todo!()
    }
}
