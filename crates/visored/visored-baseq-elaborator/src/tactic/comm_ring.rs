use super::*;

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub(crate) fn comm_ring(
        &mut self,
        prop: VdMirExprFld<'sess>,
    ) -> VdBsqHypothesisResult<'sess, AltOption<VdBsqHypothesisIdx<'sess>>> {
        Ok(self.hypothesis_constructor.assumption(prop).into())
    }
}
