use crate::*;
use husky_ast::{Ast, AstSheet, AstTokenIdxRangeSheet};
use husky_entity_tree::{EntityTreeResult, ModuleSymbolContext};
use husky_token::TokenSheetData;
use vec_like::VecPairMap;
pub(crate) struct DefnCollector<'a> {
    db: &'a dyn DefnDb,
    module_symbol_context: ModuleSymbolContext<'a>,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    ast_range_sheet: &'a AstTokenIdxRangeSheet,
    decl_sheet: DeclSheet<'a>,
}

impl<'a> DefnCollector<'a> {
    pub(crate) fn new(db: &'a dyn DefnDb, module_path: ModulePath) -> EntityTreeResult<Self> {
        let module_symbol_context = db.module_symbol_context(module_path)?;
        Ok(Self {
            db,
            module_symbol_context,
            token_sheet_data: db.token_sheet_data(module_path)?,
            ast_sheet: db.ast_sheet(module_path)?,
            ast_range_sheet: db.ast_token_idx_range_sheet(module_path)?,
            decl_sheet: db.decl_sheet(module_path)?,
        })
    }

    pub(crate) fn collect_all(self) -> DefnSheet<'a> {
        DefnSheet::new(
            self.decl_sheet
                .decls()
                .iter()
                .copied()
                .map(|(path, decl)| (path.defn_region_path(), decl.map(|decl| decl.defn(self.db))))
                .collect(),
        )
    }
}

pub(crate) fn expr_parser<'a>(
    db: &'a dyn DefnDb,
    expr_path: DefnRegionPath,
    decl_expr_region: Option<ExprRegion>,
    allow_self_type: AllowSelfType,
    allow_self_value: AllowSelfValue,
) -> BlockExprParser<'a> {
    let module_path = expr_path.module_path(db);
    let parser = ExprParser::new(
        db,
        expr_path.into(),
        db.token_sheet_data(module_path).unwrap(),
        db.module_symbol_context(module_path).unwrap(),
        decl_expr_region,
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
