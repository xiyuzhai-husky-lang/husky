use super::*;

impl EtherealTerm {
    #[inline(always)]
    pub(crate) fn apply_unchecked(
        self,
        db: &::salsa::Db,
        argument: impl Into<EtherealTerm>,
        shift: u8,
    ) -> Self {
        EtherealTermApplication::new_reduced(db, self, argument.into(), shift)
    }
}
