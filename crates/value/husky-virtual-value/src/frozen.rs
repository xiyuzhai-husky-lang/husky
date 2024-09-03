use crate::*;
use husky_value::IsFrozenValue;
use value::Value;

impl IsFrozenValue for Value {
    type Value = Value;

    fn thaw(&self) -> ((), Value) {
        todo!()
    }
}
