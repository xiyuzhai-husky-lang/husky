use super::*;

impl<'a> ExprTypeEngine<'a> {
    #[inline(always)]
    pub(super) fn expect_unit(&self) -> ExpectImplicitlyConvertible {
        ExpectImplicitlyConvertible {
            destination: self.reduced_term_menu.unit().into(),
        }
    }

    #[inline(always)]
    pub(super) fn expect_implicitly_convertible_to_bool(&self) -> ExpectImplicitlyConvertible {
        ExpectImplicitlyConvertible {
            destination: self.reduced_term_menu.bool().into(),
        }
    }
}
