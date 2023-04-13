use super::*;

impl<'a> ExprTypeEngine<'a> {
    #[inline(always)]
    pub(super) fn expect_unit(&self) -> ExpectImplicitlyConvertible {
        ExpectImplicitlyConvertible::new_pure_unit(self)
    }

    #[inline(always)]
    pub(super) fn expect_argument_ty_bool(&self) -> ExpectImplicitlyConvertible {
        ExpectImplicitlyConvertible::new_pure_bool(self)
    }
}
