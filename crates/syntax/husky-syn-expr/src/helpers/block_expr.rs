use super::*;

pub fn parse_defn_block_expr<P>(
    syn_node_path: P,
    decl_expr_region: SynExprRegion,
    allow_self_type: AllowSelfType,
    allow_self_value: AllowSelfValue,
    db: &dyn SynExprDb,
) -> Option<(SynExprIdx, SynExprRegion)>
where
    P: HasSynDefnTokraRegion + Into<ItemSynNodePath>,
{
    todo!()
}
