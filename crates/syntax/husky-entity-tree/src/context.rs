use crate::*;
use husky_word::{IdentMap, Identifier, WordDb};
use vec_like::{AsVecMapEntry, VecMap, VecMapGetEntry, VecPairMap};

pub(crate) struct EntreeSymbolContext<'a> {
    db: &'a dyn EntityTreeDb,
    crate_path: CratePath,
    crate_prelude: CrateSymbolContext<'a>,
    current_sheet: &'a EntreePresheetMut<'a>,
    sheets: &'a VecMap<EntreePresheetMut<'a>>,
}

impl<'a> EntreeSymbolContext<'a> {
    pub(crate) fn new(
        db: &'a dyn EntityTreeDb,
        crate_path: CratePath,
        crate_prelude: CrateSymbolContext<'a>,
        current_sheet: &'a EntreePresheetMut<'a>,
        sheets: &'a VecMap<EntreePresheetMut<'a>>,
    ) -> Self {
        Self {
            db,
            crate_path,
            crate_prelude,
            current_sheet,
            sheets,
        }
    }

    pub(crate) fn resolve_ident(&self, ident: Identifier) -> Option<EntitySymbol> {
        self.current_sheet
            .module_specific_symbols()
            .get_entry(ident)
            .map(|entry| entry.symbol())
            .or_else(|| self.crate_prelude.resolve_ident(ident))
    }

    pub(crate) fn db(&self) -> &dyn EntityTreeDb {
        self.db
    }

    pub(crate) fn resolve_subentity(
        &self,
        parent: EntityPath,
        ident: Identifier,
    ) -> Option<EntitySymbol> {
        let query_crate_path = parent.crate_path(self.db);
        if query_crate_path == self.crate_path {
            match parent {
                EntityPath::Module(module_path) => {
                    let module_sheet = &self.sheets[module_path];
                    module_sheet
                        .module_specific_symbols()
                        .get_entry(ident)
                        .map(|entry| entry.symbol())
                }
                EntityPath::ModuleItem(_) => todo!(),
                EntityPath::AssociatedItem(_) => todo!(),
                EntityPath::Variant(_) => todo!(),
            }
        } else {
            todo!()
        }
    }

    pub(crate) fn module_symbols(&self, module_path: ModulePath) -> &'a [EntitySymbolEntry] {
        let query_crate_path = module_path.crate_path(self.db);
        if query_crate_path == self.crate_path {
            self.sheets[module_path].module_specific_symbols()
        } else {
            todo!()
        }
    }
}
