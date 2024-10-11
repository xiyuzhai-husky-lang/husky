use crate::*;

pub trait IsLabel: __FromValue + PartialEq + Eq + Copy + __Thawed<Frozen = Self> + 'static {
    fn label() -> Self;
}
