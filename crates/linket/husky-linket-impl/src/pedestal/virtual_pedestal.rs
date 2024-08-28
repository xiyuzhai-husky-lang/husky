use super::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VirtualPedestal;

impl FromIterator<(ItemPathIdInterface, ())> for VirtualPedestal {
    fn from_iter<T: IntoIterator<Item = (ItemPathIdInterface, ())>>(iter: T) -> Self {
        Self
    }
}

impl IsPedestal for VirtualPedestal {
    type VarId = ();

    fn exclude<V: IsStaticVar<()>>(self) -> Self {
        Self
    }

    fn is_closed(&self, var_deps: &[ItemPathIdInterface]) -> bool {
        true
    }
}
