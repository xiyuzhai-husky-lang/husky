use super::*;

impl<'a> SemaExprEngine<'a> {
    #[inline(always)]
    pub(super) fn expect_ty0_subtype(&self) -> ExpectSubtype {
        ExpectSubtype::new(self.term_menu().ty0().into())
    }
}
