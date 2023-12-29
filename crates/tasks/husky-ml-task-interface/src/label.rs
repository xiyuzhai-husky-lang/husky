use super::*;
use husky_standard_value::ugly::*;

pub trait IsLabel: __FromValue + PartialEq + Eq + Copy + __Static<Frozen = Self> + 'static {
    fn label() -> Self;
    fn label_at_input(input_id: InputId) -> Self;
}
