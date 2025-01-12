use super::*;

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub(crate) fn kurapika(&mut self, prop: VdBsqExprFld<'sess>) -> Mhr<'sess> {
        self.with_call(VdBsqTacticCall::Kurapika, |slf| slf.kurapika_inner(prop))
    }

    fn kurapika_inner(&mut self, prop: VdBsqExprFld<'sess>) -> Mhr<'sess> {
        // blank implementation
        AltNothing
    }
}
