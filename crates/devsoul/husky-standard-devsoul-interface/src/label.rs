use super::*;
use husky_standard_value::ugly::*;

// todo: move this to ml-task
pub trait IsLabel: __FromValue + PartialEq + Eq + Copy + __Static<Frozen = Self> + 'static {
    fn label() -> Self;
}
