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
        debug_assert!(
            self.hypothesis_constructor
                .stack()
                .get_active_hypothesis_with_expr(prop)
                .is_none(),
            "term_trivial should only be called on a fresh prop"
        );
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
