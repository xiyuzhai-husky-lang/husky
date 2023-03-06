use super::*;

impl RawTerm {
    pub(super) fn is_ins_sort(self, db: &dyn RawTermDb) -> RawTermResult<bool> {
        Ok(match self.ty(db)? {
            Left(RawTerm::Category(_)) => true,
            Left(_) | Right(_) => false,
        })
    }
}
