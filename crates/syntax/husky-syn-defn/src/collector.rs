use crate::*;
use husky_ast::{Ast, AstSheet, AstTokenIdxRangeSheet};
use husky_entity_tree::{EntityTreeResult, ModuleSymbolContext};
use husky_token::TokenSheetData;
use vec_like::VecPairMap;

#[inline(always)]
pub(crate) fn expr_parser<'a>(
    db: &'a dyn SynDefnDb,
    node_path: impl Into<EntitySynNodePath>,
    decl_expr_region: SynExprRegion,
    allow_self_type: AllowSelfType,
    allow_self_value: AllowSelfValue,
) -> BlockExprParser<'a> {
    let node_path = node_path.into();
    let module_path = node_path.module_path(db);
    let parser = ExprParser::new(
        db,
        RegionPath::Defn(node_path.into()),
        db.token_sheet_data(module_path).unwrap(),
        db.module_symbol_context(module_path).unwrap(),
        Some(decl_expr_region),
        allow_self_type,
        allow_self_value,
    );
    BlockExprParser::new(
        parser,
        db.ast_sheet(module_path).unwrap(),
        db.ast_token_idx_range_sheet(module_path).unwrap(),
        None, // ad hoc
    )
}
