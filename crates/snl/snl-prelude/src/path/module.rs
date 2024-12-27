use super::*;
use crate::coword::SnlIdent;

#[eterned::eterned]
pub struct SnlModulePath {
    data: SnlModulePathData,
}

impl Serialize for SnlModulePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
        // serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SnlModulePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SnlModulePathData {
    Root,
    Child {
        parent: SnlModulePath,
        ident: SnlIdent,
    },
}

impl std::fmt::Debug for SnlModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        // write!(f, "{:?}", self.data)
    }
}
