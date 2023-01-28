use crate::*;

pub(crate) struct ExprTypeEngine<'a> {
    expr_region_data: &'a ExprRegionData,
    expr_ty_infos: ExprMap<ExprTypeInfo>,
}

impl<'a> ExprTypeEngine<'a> {
    pub(crate) fn new(db: &'a dyn ExprTypeDb, expr_region: ExprRegion) -> Self {
        let expr_region_data = expr_region.data(db);
        Self {
            expr_region_data,
            expr_ty_infos: ExprMap::new(expr_region_data.expr_arena()),
        }
    }

    pub(crate) fn infer_all(&mut self) {
        todo!()
    }

    pub(crate) fn finish(self) -> ExprTypeRegion {
        todo!()
    }
}
