use crate::*;
use husky_entity_tree::{ModuleSymbolContext, PreludeResult};
use husky_token::TokenDb;
use husky_vfs::{snippet::Snippet, CratePath, ModulePath};

#[salsa::tracked(jar = SynExprJar, return_ref)]
pub fn parse_expr_from_snippet(
    db: &::salsa::Db,
    crate_path: CratePath,
    snippet: Snippet,
) -> PreludeResult<(SynExprRegion, Option<SynExprIdx>)> {
    let token_sheet_data = db.snippet_token_sheet_data(snippet);
    let expr_context = SynExprContext::new2(
        db,
        SynNodeRegionPath::Snippet(ModulePath::new_snippet(snippet, db)),
        ModuleSymbolContext::new_default(db, crate_path)?,
        None,
        AllowSelfType::False,
        AllowSelfValue::False,
    )
    .unwrap();
    let token_stream =
        RegionalTokenStream::new_snippet_regional_token_stream(token_sheet_data.tokens());
    let mut expr_parser = expr_context.token_stream_expr_parser(None, token_stream);
    let expr = expr_parser.parse_expr_root(None, SynExprRootKind::Snippet);
    Ok((expr_parser.finish(), expr))
}
