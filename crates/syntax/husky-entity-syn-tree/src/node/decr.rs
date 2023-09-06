use super::*;

// basically a wrapper type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub struct DecrSynNodePath {
    path: DecrPath,
}
