use super::*;
use crate::{
    hypothesis::construction::VdBsqHypothesisConstruction,
    term::{prop::VdBsqPropTerm, VdBsqTerm},
};
use alt_option::*;

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub(crate) fn term_trivial(
        &mut self,
        prop: VdMirExprFld<'sess>,
    ) -> VdBsqHypothesisResult<'sess, AltOption<VdBsqHypothesisIdx<'sess>>> {
        let VdBsqTerm::Prop(VdBsqPropTerm::Trivial(b)) = prop.term() else {
            return Ok(AltNone);
        };
        match b {
            true => Ok(AltSome(
                self.hypothesis_constructor
                    .construct_new_hypothesis(prop, VdBsqHypothesisConstruction::TermTrivial(b)),
            )),
            false => todo!("raise error"),
        }
    }
}
