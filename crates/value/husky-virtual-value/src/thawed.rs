use crate::*;
use husky_value::IsThawedValue;
use value::Value;

impl IsThawedValue for Value {
    type Value = Value;

    fn r#move(&mut self) -> Self {
        unreachable!()
    }
}
