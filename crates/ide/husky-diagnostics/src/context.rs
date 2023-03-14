use crate::*;
use husky_ast::AstTokenIdxRangeSheet;
use husky_expr::{ExprIdx, ExprRangeRegion, ExprRegion, ExprRegionData};
use husky_expr_ty::*;
use husky_token::TokenIdxRange;

pub(crate) struct SheetDiagnosticsContext<'a> {
    db: &'a dyn DiagnosticsDb,
    token_sheet_data: &'a TokenSheetData,
    ranged_token_sheet: &'a RangedTokenSheet,
    ast_token_idx_range_sheet: &'a AstTokenIdxRangeSheet,
}

impl<'a> SheetDiagnosticsContext<'a> {
    pub(crate) fn new(db: &'a dyn DiagnosticsDb, module_path: ModulePath) -> Self {
        let ranged_token_sheet = db.ranged_token_sheet(module_path).unwrap();
        let token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        Self {
            db,
            token_sheet_data,
            ranged_token_sheet,
            ast_token_idx_range_sheet: db.ast_token_idx_range_sheet(module_path).unwrap(),
        }
    }

    pub(crate) fn db(&self) -> &dyn DiagnosticsDb {
        self.db
    }

    pub(crate) fn token_sheet_data(&self) -> &TokenSheetData {
        self.token_sheet_data
    }

    pub(crate) fn ranged_token_sheet(&self) -> &RangedTokenSheet {
        self.ranged_token_sheet
    }
}

pub(crate) struct RegionDiagnosticsContext<'a> {
    db: &'a dyn DiagnosticsDb,
    token_sheet_data: &'a TokenSheetData,
    ranged_token_sheet: &'a RangedTokenSheet,
    expr_region_data: &'a ExprRegionData,
    expr_ty_region: &'a ExprTypeRegion,
    expr_range_region: &'a ExprRangeRegion,
}

impl<'a> RegionDiagnosticsContext<'a> {
    pub(crate) fn new(db: &'a dyn DiagnosticsDb, expr_region: ExprRegion) -> Self {
        let expr_region_data = &expr_region.data(db);
        let module_path = expr_region_data.path().module_path(db);
        let ranged_token_sheet = db.ranged_token_sheet(module_path).unwrap();
        let token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        Self {
            db,
            token_sheet_data,
            ranged_token_sheet,
            expr_region_data,
            expr_ty_region: db.expr_ty_region(expr_region),
            expr_range_region: db.expr_range_region(expr_region),
        }
    }

    pub(crate) fn db(&self) -> &'a dyn DiagnosticsDb {
        self.db
    }

    pub(crate) fn token_sheet_data(&self) -> &TokenSheetData {
        self.token_sheet_data
    }

    pub(crate) fn ranged_token_sheet(&self) -> &RangedTokenSheet {
        self.ranged_token_sheet
    }

    pub(crate) fn expr_ty_region(&self) -> &ExprTypeRegion {
        self.expr_ty_region
    }

    pub(crate) fn expr_text_range(&self, expr_idx: ExprIdx) -> TextRange {
        self.text_range(self.expr_range_region[expr_idx])
    }

    fn text_range(&self, token_idx_range: TokenIdxRange) -> TextRange {
        assert!(token_idx_range.start().token_idx() < token_idx_range.end().token_idx());
        let first = self
            .ranged_token_sheet
            .token_text_range(token_idx_range.start().token_idx());
        let last = self
            .ranged_token_sheet
            .token_text_range(token_idx_range.end().token_idx() - 1);
        first.join(last)
    }
}
