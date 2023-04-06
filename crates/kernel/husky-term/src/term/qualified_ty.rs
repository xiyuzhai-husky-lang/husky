use super::*;

#[salsa::interned(db = TermDb, jar = TermJar, constructor = new_inner)]
pub struct TermQualifiedType {
    pub base_ty: Term,
}

impl TermQualifiedType {
    pub(super) fn from_raw_unchecked(
        db: &dyn TermDb,
        qualified_ty: RawTermQualifiedType,
    ) -> TermResult<Self> {
        let base_ty = Term::ty_from_raw_unchecked(db, qualified_ty.base_ty(db))?;
        Ok(Self::new_inner(db, base_ty))
    }

    pub(super) fn check(self, db: &dyn TermDb) -> TermResult<()> {
        self.base_ty(db).check(db)
    }

    pub fn toolchain(self, db: &dyn TermDb) -> Option<Toolchain> {
        self.base_ty(db).toolchain(db)
    }
}
