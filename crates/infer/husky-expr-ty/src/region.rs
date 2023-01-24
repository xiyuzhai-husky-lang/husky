use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ExprTypeRegion {}

impl<Db: ExprTypeDb + ?Sized> salsa::DebugWithDb<Db> for ExprTypeRegion {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}

#[salsa::tracked(jar = ExprTypeJar, return_ref)]
pub(crate) fn expr_ty_region(db: &dyn ExprTypeDb, expr_region: ExprRegion) -> ExprTypeRegion {
    ExprTypeRegion {}
}
