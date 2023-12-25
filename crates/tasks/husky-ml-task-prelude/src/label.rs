use super::*;
use husky_standard_value::ugly::__FromValue;

pub trait IsLabel: __FromValue + PartialEq + Eq + Copy + 'static {
    fn label() -> Self;
    fn label_at_input(input_id: InputId) -> Self;
}
