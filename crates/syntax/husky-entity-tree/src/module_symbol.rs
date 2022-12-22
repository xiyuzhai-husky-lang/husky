use crate::*;

use husky_word::{IdentMap, Identifier, WordDb};
use salsa::DebugWithDb;
use vec_like::{AsVecMapEntry, VecMap, VecPairMap};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ModuleSymbol {
    Submodule {
        ident: Identifier,
    },
    ModuleItem {
        ident: Identifier,
        ast_idx: AstIdx,
        path: ModuleItemPath,
        variants: Option<VecMap<Identifier, ModuleItemVariant>>,
    },
}

impl salsa::DebugWithDb<dyn EntityTreeDb + '_> for ModuleSymbol {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityTreeDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        match self {
            Self::Submodule { ident } => f.debug_struct("Submodule").field("ident", ident).finish(),
            Self::ModuleItem {
                ident,
                ast_idx,
                path,
                variants,
            } => f
                .debug_struct("ModuleItem")
                .field(
                    "ident",
                    &ident.debug_with(db as &dyn WordDb, include_all_fields),
                )
                .field("ast_idx", ast_idx)
                .field("path", path)
                .field("variants", variants)
                .finish(),
        }
    }
}
impl<Db: EntityTreeDb> salsa::DebugWithDb<Db> for ModuleSymbol {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn EntityTreeDb, include_all_fields)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModuleItemVariant {
    ident: Identifier,
    ast_idx: AstIdx,
}

impl AsVecMapEntry<Identifier> for ModuleItemVariant {
    fn key(&self) -> Identifier
    where
        Identifier: Copy,
    {
        self.ident
    }

    fn key_ref(&self) -> &Identifier {
        &self.ident
    }
}

impl ModuleItemVariant {
    pub fn new(ident: Identifier, ast_idx: AstIdx) -> Self {
        Self { ident, ast_idx }
    }
}
