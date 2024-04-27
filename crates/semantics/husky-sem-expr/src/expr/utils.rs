use super::*;

impl<'a> SemExprBuilder<'a> {
    #[inline(always)]
    pub(super) fn expect_ty0_subtype(&self) -> ExpectSubtypeOrEqual {
        ExpectSubtypeOrEqual::new(self.eth_term_menu().ty0().into())
    }
}
