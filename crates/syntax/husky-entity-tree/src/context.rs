use crate::*;
use husky_word::{IdentMap, Identifier, WordDb};
use vec_like::{AsVecMapEntry, VecMap, VecMapGetEntry, VecPairMap};

pub(crate) struct EntitySymbolContext<'a> {
    db: &'a dyn EntityTreeDb,
    module_path: ModulePath,
    module_symbols: &'a [EntitySymbol],
    crate_prelude: CratePrelude<'a>,
}

impl<'a> EntitySymbolContext<'a> {
    pub(crate) fn new(
        db: &'a dyn EntityTreeDb,
        module_path: ModulePath,
        module_symbols: &'a [EntitySymbol],
        crate_prelude: CratePrelude<'a>,
    ) -> Self {
        Self {
            db,
            module_path,
            module_symbols,
            crate_prelude,
        }
    }

    pub(crate) fn resolve_ident(&self, ident: Identifier) -> Option<&EntitySymbol> {
        self.module_symbols
            .get_entry(ident)
            .or_else(|| self.crate_prelude.resolve_ident(ident))
    }
}
