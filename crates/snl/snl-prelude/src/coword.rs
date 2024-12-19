pub mod error;

use self::error::{SnlCowordError, SnlCowordResult};
use base_coword::BaseCoword;
use eterned::db::EternerDb;
use vec_like::ordered_vec_map::OrderedVecPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SnlIdent(BaseCoword);

impl SnlIdent {
    pub fn from_ref(s: &str, db: &EternerDb) -> SnlCowordResult<Self> {
        Self::check(s)?;
        Ok(Self(BaseCoword::from_ref(s, db)))
    }

    pub fn check(s: &str) -> SnlCowordResult<()> {
        if s.is_empty() {
            return Err(SnlCowordError::InvalidIdent(s.to_string()));
        }
        Ok(())
    }
}

pub type SnlIdentMap<T> = OrderedVecPairMap<SnlIdent, T>;

impl serde::Serialize for SnlIdent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for SnlIdent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(SnlIdent(BaseCoword::deserialize(deserializer)?))
    }
}
