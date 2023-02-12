use super::*;

impl<'a> ExprTypeEngine<'a> {
    #[inline(always)]
    pub(super) fn expect_unit(&self) -> ExpectImplicitConversion {
        ExpectImplicitConversion {
            destination: self.reduced_term_menu.unit().into(),
        }
    }

    #[inline(always)]
    pub(super) fn expect_bool(&self) -> ExpectImplicitConversion {
        ExpectImplicitConversion {
            destination: self.reduced_term_menu.bool().into(),
        }
    }
}
