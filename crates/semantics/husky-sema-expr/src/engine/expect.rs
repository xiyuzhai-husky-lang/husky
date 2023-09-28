use super::*;

impl<'a> SemaExprEngine<'a> {
    #[inline(always)]
    pub(super) fn expect_unit(&self) -> ExpectCoersion {
        ExpectCoersion::new_pure_unit(self)
    }

    #[inline(always)]
    pub(super) fn expect_argument_ty_bool(&self) -> ExpectCoersion {
        ExpectCoersion::new_pure_bool(self)
    }
}
