use crate::*;
use husky_ast::AstSheet;
use husky_token::TokenSheet;
use vec_like::VecPairMap;

pub(crate) struct DefnCollector<'a> {
    db: &'a dyn DefnDb,
    token_sheet: &'a TokenSheet,
    ast_sheet: &'a AstSheet,
    decl_sheet: &'a DeclSheet,
}

impl<'a> DefnCollector<'a> {
    pub(crate) fn new(db: &'a dyn DefnDb, module_path: ModulePath) -> VfsResult<Self> {
        Ok(Self {
            db,
            token_sheet: db.token_sheet(module_path)?,
            ast_sheet: db.ast_sheet(module_path).unwrap(),
            decl_sheet: db.decl_sheet(module_path).unwrap(),
        })
    }

    pub(crate) fn collect_all(mut self) -> DefnSheet {
        let mut defns: VecPairMap<EntityPath, Defn> = Default::default();
        for decl in self.decl_sheet.decls().iter() {
            todo!()
        }
        DefnSheet::new(defns)
    }
}
