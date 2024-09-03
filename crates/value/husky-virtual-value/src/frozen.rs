use crate::*;
use husky_value_interface::IsFrozenValue;
use value::Value;

impl IsFrozenValue for Value {
    type Value = Value;

    fn thaw(&self) -> ((), Value) {
        todo!()
    }
}
