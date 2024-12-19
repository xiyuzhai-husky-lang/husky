use crate::{coword::SnlIdent, path::module::SnlModulePath};
use serde::{Deserialize, Serialize};

#[eterned::eterned]
pub struct SnlRewriteRulePath {
    pub module: SnlModulePath,
    pub ident: SnlIdent,
}

impl std::fmt::Debug for SnlRewriteRulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        // write!(f, "{:?}", self.data)
    }
}

impl Serialize for SnlRewriteRulePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for SnlRewriteRulePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}
