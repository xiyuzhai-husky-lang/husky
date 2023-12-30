use serde::Serialize;

use crate::IsSerdeImpl;

pub struct SerdeJson;

impl IsSerdeImpl for SerdeJson {
    type Error = serde_json::Error;

    type Value = serde_json::Value;

    fn to_value<T: Serialize>(t: T) -> Result<Self::Value, Self::Error> {
        serde_json::to_value(t)
    }

    fn to_string<T: Serialize>(t: &T) -> Result<String, Self::Error> {
        serde_json::to_string(t)
    }

    fn from_str<T: for<'a> serde::Deserialize<'a>>(s: &str) -> Result<T, Self::Error> {
        serde_json::from_str(s)
    }
}
