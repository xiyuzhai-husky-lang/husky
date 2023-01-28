use crate::*;

pub(crate) struct ExprTypeEngine<'a> {
    expr_region_data: &'a ExprRegionData,
    expr_ty_infos: ExprMap<ExprTypeResult<ExprTypeInfo>>,
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
        for root in self.expr_region_data.roots() {
            let ty = self.infer_new(root.expr());
            // todo: check coherence
        }
    }

    fn infer_new(&mut self, expr_idx: ExprIdx) -> ExprTypeResult<LocalTerm> {
        let info_result = self.calc(expr_idx);
        let ty_result = match info_result {
            Ok(ref info) => Ok(info.ty()),
            Err(_) => Err(DerivedExprTypeError::TypeInfoErr.into()),
        };
        self.save(expr_idx, info_result);
        ty_result
    }

    fn save(&mut self, expr_idx: ExprIdx, result: ExprTypeResult<ExprTypeInfo>) {
        self.expr_ty_infos.insert_new(expr_idx, result)
    }

    pub(crate) fn finish(self) -> ExprTypeRegion {
        ExprTypeRegion::new(self.expr_region_data.path(), self.expr_ty_infos)
    }

    fn calc(&mut self, expr_idx: ExprIdx) -> ExprTypeResult<ExprTypeInfo> {
        todo!()
    }
}
