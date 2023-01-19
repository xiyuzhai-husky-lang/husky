use crate::*;
use husky_entity_tree::{CrateSymbolContext, ModuleSymbolContext, PreludeResult};
use husky_vfs::CratePath;

#[salsa::tracked(jar = ExprJar, return_ref)]
pub(crate) fn parse_expr_from_snippet(
    db: &dyn ExprDb,
    crate_path: CratePath,
    snippet: Snippet,
) -> PreludeResult<(ExprPage, Option<ExprIdx>)> {
    let token_sheet_data = db.snippet_token_sheet_data(snippet);
    let token_iter = token_sheet_data
        .token_group_token_stream(token_sheet_data.token_group_iter().next().unwrap().0, None);
    let mut expr_parser = ExprParser::new(
        db,
        ExprPath::Snippet(crate_path.toolchain(db)),
        token_sheet_data,
        SymbolContextMut::new(
            ModuleSymbolContext::new_default(db, crate_path)?,
            None,
            AllowSelfType::False,
            AllowSelfValue::False,
        ),
    );
    let expr = expr_parser
        .ctx(token_iter)
        .parse_expr(ExprParseEnvironment::None);
    Ok((expr_parser.finish(), expr))
}
