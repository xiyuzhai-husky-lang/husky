use crate::*;

pub(crate) struct StackValue {
    regular_value: RegularValue,
    snapshot: Option<__RegularValueSnapshot>,
}
