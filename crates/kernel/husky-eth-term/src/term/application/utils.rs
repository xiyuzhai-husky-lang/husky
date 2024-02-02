use super::*;

impl EthTerm {
    #[inline(always)]
    pub(crate) fn apply_unchecked(
        self,
        db: &::salsa::Db,
        argument: impl Into<EthTerm>,
        shift: u8,
    ) -> Self {
        ApplicationEthTerm::new_reduced(db, self, argument.into(), shift)
    }
}
