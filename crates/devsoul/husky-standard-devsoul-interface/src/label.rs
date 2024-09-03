// ad hoc
// todo: move this to ml-task
use super::*;
#[cfg(feature = "ugly")]
use husky_standard_value::ugly::*;

#[cfg(feature = "ugly")]
pub trait IsLabel: __FromValue + PartialEq + Eq + Copy + __Thawed<Frozen = Self> + 'static {
    fn label() -> Self;
}
