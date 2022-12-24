use crate::*;
use husky_ast::AstSheet;
use husky_token::TokenSheet;

pub(crate) struct DeclCollector<'a> {
    db: &'a dyn DeclDb,
    token_sheet: &'a TokenSheet,
    ast_sheet: &'a AstSheet,
}

impl<'a> DeclCollector<'a> {
    pub(crate) fn new(db: &'a dyn DeclDb, module_path: ModulePath) -> VfsResult<Self> {
        Ok(Self {
            db,
            token_sheet: db.token_sheet(module_path)?,
            ast_sheet: db.ast_sheet(module_path).unwrap(),
        })
    }

    pub(crate) fn collect_all(mut self) -> DeclSheet {
        for (ast_idx, ast) in self.ast_sheet.indexed_asts() {
            todo!()
        }
        todo!()
    }
}
