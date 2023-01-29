use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb, jar = ExprTypeJar)]
pub struct ExprTypeRegion {
    path: RegionPath,
    expr_ty_infos: ExprMap<ExprTypeInfo>,
}

impl ExprTypeRegion {
    pub(crate) fn new(path: RegionPath, expr_ty_infos: ExprMap<ExprTypeInfo>) -> Self {
        Self {
            path,
            expr_ty_infos,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb, jar = ExprTypeJar)]
pub(crate) struct ExprTypeInfo {
    ty_result: ExprTypeResult<LocalTerm>,
    opt_expectation: OptionExpectationIdx,
}

impl ExprTypeInfo {
    pub(crate) fn new(
        ty_result: ExprTypeResult<LocalTerm>,
        opt_expectation: OptionExpectationIdx,
    ) -> Self {
        Self {
            ty_result,
            opt_expectation,
        }
    }

    pub(crate) fn ty(&self) -> ExprTypeResultRef<LocalTerm> {
        self.ty_result.as_ref().map(|t| *t)
    }
}

#[salsa::tracked(jar = ExprTypeJar, return_ref)]
pub(crate) fn expr_ty_region(db: &dyn ExprTypeDb, expr_region: ExprRegion) -> ExprTypeRegion {
    let mut engine = ExprTypeEngine::new(db, expr_region);
    engine.infer_all();
    engine.finish()
}
