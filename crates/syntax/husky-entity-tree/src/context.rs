use crate::*;

use husky_word::Ident;
use vec_like::VecMap;

pub(crate) struct EntityTreeSymbolContext<'a> {
    db: &'a dyn EntityTreeDb,
    crate_path: CratePath,
    crate_root: ModulePath,
    crate_prelude: CrateSymbolContext<'a>,
    current_sheet: &'a EntityTreePresheetMut<'a>,
    sheets: &'a VecMap<EntityTreePresheetMut<'a>>,
}

impl<'a> EntityTreeSymbolContext<'a> {
    pub(crate) fn new(
        db: &'a dyn EntityTreeDb,
        crate_path: CratePath,
        crate_root: ModulePath,
        crate_prelude: CrateSymbolContext<'a>,
        current_sheet: &'a EntityTreePresheetMut<'a>,
        sheets: &'a VecMap<EntityTreePresheetMut<'a>>,
    ) -> Self {
        Self {
            db,
            crate_path,
            crate_root,
            crate_prelude,
            current_sheet,
            sheets,
        }
    }

    pub(crate) fn resolve_ident(&self, ident: Ident) -> Option<EntitySymbol> {
        self.current_sheet
            .module_specific_symbols()
            .resolve_ident(ident)
            .or_else(|| self.crate_prelude.resolve_ident(ident))
    }

    pub(crate) fn db(&self) -> &dyn EntityTreeDb {
        self.db
    }

    pub(crate) fn resolve_subentity(
        &self,
        parent: EntityPath,
        ident: Ident,
    ) -> Option<EntitySymbol> {
        let query_crate_path = parent.crate_path(self.db);
        // here we divide into two cases
        // - query_crate_path is self. We have to use self.sheets
        // - query_crate_path is package dependency.
        //      This is easy, we just use db, because it's guaranteed that there will be no cycle.
        if query_crate_path == self.crate_path {
            match parent {
                EntityPath::Module(module_path) => {
                    // 如果出现 unwrap None的错误，就是因为module_path对应的文件不存在
                    // 后面应该通过某些东西保证每个module_path对应的文件都存在
                    let module_sheet = &self.sheets[module_path];
                    module_sheet.module_specific_symbols().resolve_ident(ident)
                }
                EntityPath::ModuleItem(_) => todo!(),
                EntityPath::AssociatedItem(_) => todo!(),
                EntityPath::Variant(_) => todo!(),
            }
        } else {
            self.db
                .subentity_path(parent, ident)
                .ok()
                .map(|entity_path| EntitySymbol::PackageDependency { entity_path })
        }
    }

    pub(crate) fn module_symbols(&self, module_path: ModulePath) -> EntitySymbolTableRef<'a> {
        let query_crate_path = module_path.crate_path(self.db);
        if query_crate_path == self.crate_path {
            self.sheets[module_path].module_specific_symbols()
        } else {
            todo!()
        }
    }

    pub(crate) fn crate_root(&self) -> ModulePath {
        self.crate_root
    }
}
