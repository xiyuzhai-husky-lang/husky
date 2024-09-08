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

    fn var_ids<'a>(&'a self) -> impl Iterator<Item = (ItemPathIdInterface, Self::VarId)> + 'a {
        [].into_iter()
    }

    fn insert(&mut self, item_path_id_interface: ItemPathIdInterface, var_id: Self::VarId) {
        todo!()
    }
}
