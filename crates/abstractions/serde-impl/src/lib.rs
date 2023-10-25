use serde::{Deserialize, Serialize};

pub trait IsSerdeImpl {
    type Error: Send + 'static;

    fn to_string<T: Serialize>(t: &T) -> Result<String, Self::Error>;

    fn from_str<T: for<'a> Deserialize<'a>>(s: &str) -> Result<T, Self::Error>;
}
