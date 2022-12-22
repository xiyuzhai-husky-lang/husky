use crate::*;

use husky_print_utils::p;
use husky_word::{IdentMap, Identifier, WordDb};
use salsa::DebugWithDb;
use vec_like::{AsVecMapEntry, VecMap, VecMapGetEntry, VecPairMap};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModuleItemVariant {
    ident: Identifier,
    ast_idx: AstIdx,
}

impl AsVecMapEntry for ModuleItemVariant {
    type K = Identifier;

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

pub(crate) struct EntitySymbolContext<'a> {
    db: &'a dyn EntityTreeDb,
    module_path: ModulePath,
    nodes: &'a [EntityNode],
    crate_prelude: CratePrelude<'a>,
}

impl<'a> EntitySymbolContext<'a> {
    pub(crate) fn new(
        db: &'a dyn EntityTreeDb,
        module_path: ModulePath,
        nodes: &'a [EntityNode],
        crate_prelude: CratePrelude<'a>,
    ) -> Self {
        Self {
            db,
            module_path,
            nodes,
            crate_prelude,
        }
    }

    pub(crate) fn get(&self, ident: Identifier) -> Option<&EntityNode> {
        self.nodes
            .get_entry(ident)
            .or_else(|| self.crate_prelude.get(ident))
    }
}

pub(crate) struct CratePrelude<'a> {
    universal_prelude: &'a [EntityNode],
    crate_specific_prelude: &'a [EntityNode],
}

impl<'a> CratePrelude<'a> {
    pub(crate) fn new(
        universal_prelude: &'a [EntityNode],
        crate_specific_prelude: &'a [EntityNode],
    ) -> Self {
        Self {
            universal_prelude,
            crate_specific_prelude,
        }
    }

    fn get(&self, ident: Identifier) -> Option<&'a EntityNode> {
        self.universal_prelude
            .get_entry(ident)
            .or_else(|| self.crate_specific_prelude.get_entry(ident))
    }
}
