use crate::*;
use husky_ast::*;
use husky_token::TokenSheet;

pub(crate) struct DefnParser<'a> {
    db: &'a dyn DefnDb,
    token_sheet: &'a TokenSheet,
    ast_sheet: &'a AstSheet,
    arena: ExprArena,
}

impl<'a> DefnParser<'a> {
    pub(crate) fn new(
        db: &'a dyn DefnDb,
        token_sheet: &'a TokenSheet,
        ast_sheet: &'a AstSheet,
        decl_sheet: &'a DeclSheet,
        ast_idx: AstIdx,
        ast: &Ast,
    ) -> Option<Self> {
        let decl = todo!();
        Some(Self {
            db,
            token_sheet,
            ast_sheet,
            arena: Default::default(),
        })
    }

    pub(crate) fn collect_all(mut self) -> DefnSheet {
        for (ast_idx, ast) in self.ast_sheet.indexed_asts() {
            todo!()
        }
        todo!()
    }
}
