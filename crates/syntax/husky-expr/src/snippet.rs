use crate::*;
use husky_entity_tree::PreludeResult;
use husky_vfs::CratePath;

#[salsa::tracked(jar = ExprJar, return_ref)]
pub(crate) fn parse_expr_from_snippet(
    db: &dyn ExprDb,
    crate_path: CratePath,
    snippet: Snippet,
) -> PreludeResult<(ExprSheet, Option<ExprIdx>)> {
    let crate_prelude = db.crate_prelude(crate_path)?;
    let token_sheet = db.tokenize_snippet(snippet);
    let mut parser = ExprParser::new(db, &token_sheet, None, crate_prelude);
    let token_iter = token_sheet
        .token_group_token_stream(token_sheet.token_group_iter().next().unwrap().0, None);
    Ok(parse_expr(
        db,
        crate_prelude,
        &token_sheet,
        token_iter,
        ExprParseEnvironment::None,
    ))
}
