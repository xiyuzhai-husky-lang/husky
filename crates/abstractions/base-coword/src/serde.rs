use crate::BaseCoword;
use ::serde::{Deserialize, Serialize};
use eterned::attach::with_attached_eterner_db;

impl Serialize for BaseCoword {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.data().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BaseCoword {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        with_attached_eterner_db(|db| Ok(BaseCoword::from_ref(&s, db)))
            .expect("no database attached")
    }
}
