use crate::*;
use husky_coword::Ident;
use husky_entity_kind::TypeKind;
use husky_token::IdentToken;
use vec_like::VecMap;

// can see other modules in the crate
pub(crate) struct EntityTreeSymbolContext<'a, 'b>
where
    'a: 'b,
{
    db: &'a ::salsa::Db,
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
        db: &'a ::salsa::Db,
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

    pub(crate) fn db(&self) -> &'a ::salsa::Db {
        self.db
    }

    // associated items not included
    pub(crate) fn resolve_subitem_symbol(
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
                    // If an 'unwrap None' error occurs, it is because the file corresponding to the module_path does not exist.
                    // Later, something should ensure that a file exists for each module_path.
                    let module_sheet = &self.presheets[module_path];
                    module_sheet.module_specific_symbols().resolve_ident(
                        self.db(),
                        self.current_sheet.module_path().into(),
                        ident,
                    )
                }
                MajorEntityPath::MajorItem(path) => match path {
                    MajorItemPath::Type(path) => {
                        let db = self.db;
                        match path.ty_kind(db) {
                            TypeKind::Enum | TypeKind::Inductive => path
                                .ty_variant_paths(db)
                                .iter()
                                .find_map(|&(ident1, ty_variant_path)| {
                                    (ident == ident1)
                                        .then_some(EntitySymbol::TypeVariant { ty_variant_path })
                                }),
                            TypeKind::Record
                            | TypeKind::Struct
                            | TypeKind::Structure
                            | TypeKind::Extern => None,
                        }
                    }
                    MajorItemPath::Trait(_) => None,
                    MajorItemPath::Form(_) => None,
                },
            }
        } else {
            match self.db.subitem_path(parent, ident).ok()? {
                SubitemPath::Principal(item_path) => {
                    // ad hoc
                    Some(EntitySymbol::PackageDependency { item_path })
                }
                SubitemPath::Assoc => None,
            }
        }
    }

    pub(crate) fn crate_root(&self) -> ModulePath {
        self.crate_root
    }

    pub(crate) fn presheets(&self) -> &VecMap<EntityTreePresheetMut> {
        self.presheets
    }
}
