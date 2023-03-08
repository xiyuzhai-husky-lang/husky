use crate::*;

pub trait RawTypeDb: salsa::DbWithJar<RawTypeJar> + SignatureDb {
    fn ty_method_raw_ty(&self, raw_ty: RawTerm, ident: Ident) -> RawTypeResult<Option<RawTerm>>;
    fn field_raw_ty(&self, raw_ty: RawTerm, ident: Ident) -> RawTypeResult<Option<RawTerm>>;
}

impl<Db> RawTypeDb for Db
where
    Db: salsa::DbWithJar<RawTypeJar> + SignatureDb,
{
    fn ty_method_raw_ty(&self, raw_ty: RawTerm, ident: Ident) -> RawTypeResult<Option<RawTerm>> {
        raw_ty_method_raw_ty(self, raw_ty, ident)
    }

    fn field_raw_ty(&self, raw_ty: RawTerm, ident: Ident) -> RawTypeResult<Option<RawTerm>> {
        field_raw_ty(self, raw_ty, ident)
    }
}
