use crate::*;
use husky_ast::AstSheet;
use husky_token::TokenSheet;

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
        for (ast_idx, ast) in self.ast_sheet.indexed_asts() {
            if let Some(_) = DefnParser::new(
                self.db,
                self.token_sheet,
                self.ast_sheet,
                self.decl_sheet,
                ast_idx,
                ast,
            ) {
                todo!()
            }
        }
        todo!()
    }
}
