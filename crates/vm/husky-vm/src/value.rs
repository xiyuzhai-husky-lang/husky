use crate::*;

pub(crate) struct StackValue {
    regular_value: __RegularValue,
    snapshot: Option<__RegularValueSnapshot>,
}
