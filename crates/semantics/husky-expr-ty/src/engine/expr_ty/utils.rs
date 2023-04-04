use super::*;

impl<'a> ExprTypeEngine<'a> {
    #[inline(always)]
    pub(super) fn expect_eqs_ty0(&self) -> ExpectSubtype {
        ExpectSubtype::new(self.term_menu().ty0().into())
    }
}
