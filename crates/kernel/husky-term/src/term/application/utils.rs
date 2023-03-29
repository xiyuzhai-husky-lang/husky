use super::*;

impl Term {
    #[inline(always)]
    pub(crate) fn apply_unchecked(
        self,
        db: &dyn TermDb,
        argument: impl Into<Term>,
        shift: u8,
    ) -> Self {
        TermApplication::new_unchecked(db, self, argument.into(), shift)
    }
}
