use super::*;
use husky_entity_tree::{node::ItemSynNodePath, region_path::SynNodeRegionPath};

pub fn parse_defn_block_expr(
    syn_node_path: ItemSynNodePath,
    decl_expr_region: SynExprRegion,
    allow_self_type: AllowSelfType,
    allow_self_value: AllowSelfValue,
    db: &::salsa::Db,
) -> Option<(SynExprIdx, SynExprRegion)> {
    let mut ctx = SynExprContext::new(
        syn_node_path.module_path(db),
        SynNodeRegionPath::ItemDefn(syn_node_path),
        decl_expr_region,
        allow_self_type,
        allow_self_value,
        db,
    )?;
    let root_body = ctx.parse_root_body();
    let syn_expr_region = ctx.finish();
    Some((root_body, syn_expr_region))
}
