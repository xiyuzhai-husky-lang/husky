use crate::*;
use husky_value_interface::IsThawedValue;
use value::Value;

impl IsThawedValue for Value {
    type Value = Value;
}
