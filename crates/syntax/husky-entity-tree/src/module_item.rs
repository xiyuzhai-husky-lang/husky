use crate::*;

use husky_word::{IdentMap, Identifier};
use salsa::DebugWithDb;
use vec_like::{AsVecMapEntry, VecPairMap};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModuleItem {
    ident: Identifier,
    ast_idx: AstIdx,
    path: ModuleItemPath,
    variants: Option<Vec<EnumVariant>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EnumVariant {
    ident: Identifier,
    ast_idx: AstIdx,
}
