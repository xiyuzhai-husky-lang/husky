use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ExprTypeRegion {
    path: ExprRegionPath,
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
    ExprTypeRegion {
        path: expr_region.path(db),
    }
}
