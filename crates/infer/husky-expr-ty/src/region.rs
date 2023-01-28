use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ExprTypeRegion {
    path: RegionPath,
    expr_ty_infos: ExprMap<ExprTypeResult<ExprTypeInfo>>,
}

impl ExprTypeRegion {
    pub(crate) fn new(
        path: RegionPath,
        expr_ty_infos: ExprMap<ExprTypeResult<ExprTypeInfo>>,
    ) -> Self {
        Self {
            path,
            expr_ty_infos,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ExprTypeInfo {
    ty: LocalTerm,
}

impl ExprTypeInfo {
    pub(crate) fn ty(&self) -> LocalTerm {
        self.ty
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
            .finish()
    }
}

#[salsa::tracked(jar = ExprTypeJar, return_ref)]
pub(crate) fn expr_ty_region(db: &dyn ExprTypeDb, expr_region: ExprRegion) -> ExprTypeRegion {
    let mut engine = ExprTypeEngine::new(db, expr_region);
    engine.infer_all();
    engine.finish()
}
