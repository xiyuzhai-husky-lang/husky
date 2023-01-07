use crate::*;
use husky_entity_tree::{CratePrelude, PreludeResult};
use husky_vfs::CratePath;

#[salsa::tracked(jar = ExprJar, return_ref)]
pub(crate) fn parse_expr_from_snippet(
    db: &dyn ExprDb,
    crate_path: CratePath,
    snippet: Snippet,
) -> PreludeResult<(SnippetExprSheet, Option<ExprIdx>)> {
    let crate_prelude = db.crate_prelude(crate_path)?;
    let token_sheet_data = db.snippet_token_sheet_data(snippet);
    let token_iter = token_sheet_data
        .token_group_token_stream(token_sheet_data.token_group_iter().next().unwrap().0, None);
    let mut expr_parser = ExprParser::new(
        db,
        None,
        token_sheet_data,
        SnippetSymbolContextMut::new(crate_prelude),
    );
    let expr = expr_parser
        .ctx(token_iter)
        .parse_expr(ExprParseEnvironment::None);
    Ok((expr_parser.finish(), expr))
}
pub struct SnippetSymbolContextMut<'a> {
    crate_prelude: CratePrelude<'a>,
}

impl<'a> SnippetSymbolContextMut<'a> {
    pub fn new(crate_prelude: CratePrelude<'a>) -> Self {
        Self { crate_prelude }
    }
}

impl<'a> SymbolContext for SnippetSymbolContextMut<'a> {
    fn resolve_ident(&self, token_idx: TokenIdx, ident: Identifier) -> Option<Symbol> {
        if let Some(entity_symbol) = self.crate_prelude.resolve_ident(ident) {
            Some(Symbol::Entity(entity_symbol.entity_path()))
        } else {
            None
        }
    }

    fn exprs(&self) -> &[Expr] {
        todo!()
    }
}

impl<'a> SymbolContextMut for SnippetSymbolContextMut<'a> {
    type ExprSheet = SnippetExprSheet;

    fn into_expr_sheet(
        self,
        db: &dyn ExprDb,
        expr_arena: ExprArena,
        entity_path_expr_arena: EntityPathExprArena,
        pattern_expr_sheet: PatternExprSheet,
        stmt_arena: StmtArena,
    ) -> Self::ExprSheet {
        todo!()
    }
}

#[salsa::tracked(jar = ExprJar)]
pub struct SnippetExprSheet {
    #[return_ref]
    pub expr_arena: ExprArena,
    #[return_ref]
    pub entity_path_expr_arena: EntityPathExprArena,
    #[return_ref]
    pub pattern_expr_arena: PatternExprSheet,
    #[return_ref]
    pub stmt_arena: StmtArena,
}
