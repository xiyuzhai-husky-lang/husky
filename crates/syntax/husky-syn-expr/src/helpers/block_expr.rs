use super::*;

pub fn parse_defn_block_expr(
    syn_node_path: ItemSynNodePath,
    decl_expr_region: SynExprRegion,
    allow_self_type: AllowSelfType,
    allow_self_value: AllowSelfValue,
    db: &::salsa::Db,
) -> Option<(SynExprIdx, SynExprRegion)> {
    let mut stmt_context = SynStmtContext::new(
        syn_node_path,
        decl_expr_region,
        allow_self_type,
        allow_self_value,
        db,
    )?;
    let root_body = stmt_context.parse_root_body();
    let syn_expr_region = stmt_context.finish();
    Some((root_body, syn_expr_region))
}
