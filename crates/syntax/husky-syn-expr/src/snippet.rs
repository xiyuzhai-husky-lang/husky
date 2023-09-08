use crate::*;
use husky_entity_syn_tree::{CratePrelude, ModuleSymbolContext, PreludeResult};
use husky_vfs::CratePath;

#[salsa::tracked(jar = SynExprJar, return_ref)]
pub(crate) fn parse_expr_from_snippet(
    db: &dyn SynExprDb,
    crate_path: CratePath,
    snippet: Snippet,
) -> PreludeResult<(SynExprRegion, Option<SynExprIdx>)> {
    let token_sheet_data = db.snippet_token_sheet_data(snippet);
    let mut expr_context = SynExprContext::new(
        db,
        RegionPath::Snippet(crate_path.root_module_path(db)),
        ModuleSymbolContext::new_default(db, crate_path)?,
        None,
        AllowSelfType::False,
        AllowSelfValue::False,
    );
    let token_stream = token_sheet_data
        .token_group_token_stream(token_sheet_data.token_group_iter().next().unwrap().0, None);
    let mut expr_parser = expr_context.expr_parser(None, token_stream);
    let expr = expr_parser.parse_expr_root(None, ExprRootKind::Snippet);
    Ok((expr_parser.finish(), expr))
}
