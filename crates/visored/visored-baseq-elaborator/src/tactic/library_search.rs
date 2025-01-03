use super::*;
use alt_option::*;

impl<'db, 'sess> VdBaseqElaboratorInner<'db, 'sess> {
    pub(crate) fn library_search(
        &mut self,
        prop: VdMirExprFld<'sess>,
    ) -> VdBaseqHypothesisResult<'sess, AltOption<VdBaseqHypothesisIdx<'sess>>> {
        try_alt!(self.square_nonnegative(prop));
        Ok(AltNone)
    }

    fn square_nonnegative(
        &mut self,
        prop: VdMirExprFld<'sess>,
    ) -> VdBaseqHypothesisResult<'sess, AltOption<VdBaseqHypothesisIdx<'sess>>> {
        use husky_print_utils::*;
        p!(prop);
        todo!()
    }
}
