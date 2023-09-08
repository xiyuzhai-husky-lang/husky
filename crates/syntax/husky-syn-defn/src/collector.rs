use crate::*;
use husky_ast::{Ast, AstSheet, AstTokenIdxRangeSheet};
use husky_entity_syn_tree::{EntitySynTreeResult, ModuleSymbolContext};
use husky_token::TokenSheetData;
use husky_vfs::HasModulePath;
use vec_like::VecPairMap;

#[inline(always)]
pub(crate) fn stmt_context<'a>(
    db: &'a dyn SynDefnDb,
    syn_node_path: impl Into<ItemSynNodePath>,
    decl_expr_region: SynExprRegion,
    allow_self_type: AllowSelfType,
    allow_self_value: AllowSelfValue,
) -> StmtContext<'a> {
    let syn_node_path = syn_node_path.into();
    let module_path = syn_node_path.module_path(db);
    let parser = SynExprContext::new(
        db,
        RegionPath::Defn(syn_node_path.into()),
        db.module_symbol_context(module_path).unwrap(),
        Some(decl_expr_region),
        allow_self_type,
        allow_self_value,
    );
    StmtContext::new(parser, module_path, db)
}
