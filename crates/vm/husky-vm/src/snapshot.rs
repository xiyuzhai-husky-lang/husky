use husky_linket::linket::Linket;
use husky_linket_impl::linket_impl::{IsLinketImpl, LinketImplFrozenValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VmSnapshotKey {}

pub struct VmSnapshot<LinketImpl: IsLinketImpl> {
    linket: Linket,
    place_frozen_values: Vec<LinketImplFrozenValue<LinketImpl>>,
}

impl<LinketImpl: IsLinketImpl> VmSnapshot<LinketImpl> {
    pub fn linket(&self) -> Linket {
        self.linket
    }

    pub fn place_frozen_values(&self) -> &[LinketImplFrozenValue<LinketImpl>] {
        &self.place_frozen_values
    }
}
