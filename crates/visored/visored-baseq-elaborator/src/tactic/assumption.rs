use super::*;

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub(crate) fn assumption(
        &mut self,
        prop: VdMirExprFld<'sess>,
    ) -> VdBsqHypothesisResult<'sess, AltOption<VdBsqHypothesisIdx<'sess>>> {
        self.with_call(VdBsqTacticCall::Assumption, |slf| {
            Ok(slf.hypothesis_constructor.assumption(prop).into())
        })
    }
}
