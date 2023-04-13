use super::*;

impl<'a> ExprTypeEngine<'a> {
    #[inline(always)]
    pub(super) fn expect_unit(&self) -> ExpectImplicitlyConvertible {
        ExpectImplicitlyConvertible::new_pure(self.term_menu.unit().into())
    }

    #[inline(always)]
    pub(super) fn expect_argument_ty_bool(&self) -> ExpectImplicitlyConvertible {
        ExpectImplicitlyConvertible::new_pure(self.term_menu.bool_ty_ontology().into())
    }
}
