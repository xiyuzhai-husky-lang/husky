use crate::*;

use husky_coword::Ident;
use husky_token::{IdentToken, TokenIdx};
use vec_like::VecMap;

// can see other modules in the crate
pub(crate) struct EntityTreeSymbolContext<'a, 'b>
where
    'a: 'b,
{
    db: &'a dyn EntitySynTreeDb,
    crate_path: CratePath,
    crate_root: ModulePath,
    crate_prelude: CratePrelude<'b>,
    current_sheet: &'b EntityTreePresheetMut<'a>,
    presheets: &'b VecMap<EntityTreePresheetMut<'a>>,
}

impl<'a, 'b> EntityTreeSymbolContext<'a, 'b>
where
    'a: 'b,
{
    pub(crate) fn new(
        db: &'a dyn EntitySynTreeDb,
        crate_path: CratePath,
        crate_root: ModulePath,
        crate_prelude: CratePrelude<'b>,
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

    pub(crate) fn resolve_root_ident(&self, ident_token: IdentToken) -> Option<EntitySymbol> {
        let ident = ident_token.ident();
        let db = self.db;
        let module_path = self.current_sheet.module_path();
        self.current_sheet
            .module_specific_symbols()
            .resolve_ident(db, module_path.into(), ident)
            .or_else(|| self.crate_prelude.resolve_ident(db, module_path, ident))
    }

    pub(crate) fn db(&self) -> &'a dyn EntitySynTreeDb {
        self.db
    }

    // associated items not included
    pub(crate) fn resolve_subentity(
        &self,
        parent: MajorEntityPath,
        ident: Ident,
    ) -> Option<EntitySymbol> {
        let query_crate_path = parent.crate_path(self.db);
        // here we divide into two cases
        // - query_crate_path is self. We have to use self.sheets
        // - query_crate_path is package dependency.
        //      This is easy, we just use db, because it's guaranteed that there will be no cycle.
        if query_crate_path == self.crate_path {
            match parent {
                MajorEntityPath::Module(module_path) => {
                    // 如果出现 unwrap None的错误，就是因为module_path对应的文件不存在
                    // 后面应该通过某些东西保证每个module_path对应的文件都存在
                    let module_sheet = &self.presheets[module_path];
                    module_sheet.module_specific_symbols().resolve_ident(
                        self.db(),
                        self.current_sheet.module_path().into(),
                        ident,
                    )
                }
                MajorEntityPath::ModuleItem(_) => todo!(),
            }
        } else {
            match self.db.subentity_path(parent, ident).ok()? {
                SubentityPath::Principal(principal_entity_path) => {
                    Some(EntitySymbol::PackageDependency {
                        entity_path: principal_entity_path,
                    })
                }
                SubentityPath::Associated => todo!(),
            }
        }
    }

    pub(crate) fn crate_path(&self) -> CratePath {
        self.crate_path
    }

    pub(crate) fn crate_root(&self) -> ModulePath {
        self.crate_root
    }

    pub(crate) fn presheets(&self) -> &VecMap<EntityTreePresheetMut> {
        self.presheets
    }
}
