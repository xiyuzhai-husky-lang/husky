use crate::*;
use husky_expr::{ExprRegion, ExprRegionData};
use husky_expr_ty::*;

pub(crate) struct DiagnosticsSheetContext<'a> {
    db: &'a dyn DiagnosticsDb,
    token_sheet_data: &'a TokenSheetData,
    ranged_token_sheet: &'a RangedTokenSheet,
}

impl<'a> DiagnosticsSheetContext<'a> {
    pub(crate) fn new(db: &'a dyn DiagnosticsDb, module_path: ModulePath) -> Self {
        let ranged_token_sheet = db.ranged_token_sheet(module_path).unwrap();
        let token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        Self {
            db,
            token_sheet_data,
            ranged_token_sheet,
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

pub(crate) struct DiagnosticsRegionContext<'a> {
    db: &'a dyn DiagnosticsDb,
    token_sheet_data: &'a TokenSheetData,
    ranged_token_sheet: &'a RangedTokenSheet,
    expr_region_data: &'a ExprRegionData,
    expr_ty_region: &'a ExprTypeRegion,
}

impl<'a> DiagnosticsRegionContext<'a> {
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

    pub(crate) fn expr_ty_region(&self) -> &ExprTypeRegion {
        self.expr_ty_region
    }
}
