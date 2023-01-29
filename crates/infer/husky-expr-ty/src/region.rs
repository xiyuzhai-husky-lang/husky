use crate::*;

#[derive(Debug, PartialEq, Eq)]
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
pub(crate) struct ExprTypeInfo {
    ty_result: ExprTypeResult<LocalTerm>,
    opt_expectation: OptionExpectationIdx,
}

impl<Db: ?Sized> salsa::DebugWithDb<Db> for ExprTypeInfo {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
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

impl<Db: ExprTypeDb + ?Sized> salsa::DebugWithDb<Db> for ExprTypeRegion {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<ExprTypeJar>>::as_jar_db(db);
        f.debug_struct("ExprTypeRegion")
            .field("path", &self.path.debug_with(db, include_all_fields))
            .field(
                "expr_ty_infos",
                &self.expr_ty_infos.debug_with(db, include_all_fields),
            )
            .finish()
    }
}

#[salsa::tracked(jar = ExprTypeJar, return_ref)]
pub(crate) fn expr_ty_region(db: &dyn ExprTypeDb, expr_region: ExprRegion) -> ExprTypeRegion {
    let mut engine = ExprTypeEngine::new(db, expr_region);
    engine.infer_all();
    engine.finish()
}
