use super::*;

impl ValidTerm {
    pub(super) fn is_ins_sort(self, db: &dyn ValidTermDb) -> ValidTermResult<bool> {
        Ok(match self.precise_ty(db)? {
            Left(PreciseTerm::Category(_)) => true,
            Left(_) | Right(_) => false,
        })
    }
}
