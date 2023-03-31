use crate::*;

use husky_word::Ident;
use vec_like::VecMap;

pub(crate) struct EntityTreeSymbolContext<'a, 'b>
where
    'a: 'b,
{
    db: &'a dyn EntityTreeDb,
    crate_path: CratePath,
    crate_root: ModulePath,
    crate_prelude: CrateSymbolContext<'b>,
    current_sheet: &'b EntityTreePresheetMut<'a>,
    presheets: &'b VecMap<EntityTreePresheetMut<'a>>,
}

impl<'a, 'b> EntityTreeSymbolContext<'a, 'b>
where
    'a: 'b,
{
    pub(crate) fn new(
        db: &'a dyn EntityTreeDb,
        crate_path: CratePath,
        crate_root: ModulePath,
        crate_prelude: CrateSymbolContext<'b>,
        current_sheet: &'b EntityTreePresheetMut<'a>,
        sheets: &'b VecMap<EntityTreePresheetMut<'a>>,
    ) -> Self {
        Self {
            db,
            crate_path,
            crate_root,
            crate_prelude,
            current_sheet,
            presheets: sheets,
        }
    }

    pub(crate) fn resolve_ident(&self, ident: Ident) -> Option<EntitySymbol> {
        self.current_sheet
            .module_specific_symbols()
            .resolve_ident(ident)
            .or_else(|| self.crate_prelude.resolve_ident(ident))
    }

    pub(crate) fn db(&self) -> &'a dyn EntityTreeDb {
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
                    let module_sheet = &self.presheets[module_path];
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

    pub(crate) fn crate_root(&self) -> ModulePath {
        self.crate_root
    }

    pub(crate) fn presheets(&self) -> &VecMap<EntityTreePresheetMut> {
        self.presheets
    }
}
