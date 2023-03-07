use super::*;
use husky_entity_path::{EntityPathMenu, TypePath};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawTermAmbiguousTypePath {
    toolchain: Toolchain,
    kind: RawTermAmbiguousEntityPathKind,
}

impl RawTermAmbiguousTypePath {
    pub fn kind(&self) -> &RawTermAmbiguousEntityPathKind {
        &self.kind
    }

    pub fn toolchain(&self) -> Toolchain {
        self.toolchain
    }

    pub fn new_bitnot_or_leash(toolchain: Toolchain) -> Self {
        Self {
            toolchain,
            kind: todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RawTermAmbiguousEntityPathKind {
    LeashOrBitNot(TypePath),
}
