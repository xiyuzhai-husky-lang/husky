use super::*;

impl<'a> SemaExprBuilder<'a> {
    #[inline(always)]
    pub(crate) fn expect_unit(&self) -> ExpectCoercion {
        ExpectCoercion::new_pure_unit(self)
    }

    #[inline(always)]
    pub(crate) fn expect_argument_ty_bool(&self) -> ExpectCoercion {
        ExpectCoercion::new_pure_bool(self)
    }
}
