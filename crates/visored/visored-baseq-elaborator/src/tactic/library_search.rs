use super::*;
use crate::{
    coercion::VdBsqCoercionOutcome, expr::VdMirExprFldData,
    hypothesis::construction::VdBsqHypothesisConstruction,
};
use alt_option::*;
use visored_entity_path::{
    path::{
        set::{VdPreludeSetPath, VdSetPath},
        VdItemPath,
    },
    theorem::VdTheoremPath,
};
use visored_mir_expr::expr::application::VdMirFunc;
use visored_mir_opr::separator::VdMirBaseSeparator;
use visored_term::term::VdTerm;

macro_rules! require {
    ($condition:expr) => {
        if !$condition {
            return Ok(AltNone);
        }
    };
}

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub(crate) fn library_search(
        &mut self,
        prop: VdMirExprFld<'sess>,
    ) -> VdBsqHypothesisResult<'sess, AltOption<VdBsqHypothesisIdx<'sess>>> {
        try_alt!(self.square_nonnegative(prop));
        Ok(AltNone)
    }

    fn square_nonnegative(
        &mut self,
        prop: VdMirExprFld<'sess>,
    ) -> VdBsqHypothesisResult<'sess, AltOption<VdBsqHypothesisIdx<'sess>>> {
        use husky_print_utils::*;
        let VdMirExprFldData::ChainingSeparatedList {
            leader,
            followers,
            joined_separator_and_signature: None,
        } = prop.data()
        else {
            return Ok(AltNone);
        };
        assert!(followers.len() == 1);
        let (ge, rhs) = followers[0];
        let VdMirFunc::NormalBaseSeparator(ge) = ge else {
            unreachable!()
        };
        require!(ge.opr() == VdMirBaseSeparator::GE);
        require!(rhs.is_zero());
        let VdMirExprFldData::Application {
            function: pow,
            arguments: pow_args,
        } = leader.data()
        else {
            unreachable!()
        };
        let VdMirFunc::Power(pow) = pow else {
            return Ok(AltNone);
        };
        require!(pow_args[1].eqs_nat128(2));
        let Some(is_real_coercion) = pow_args[0].is_real(self).coercion() else {
            return Ok(AltNone);
        };
        let construction = VdBsqHypothesisConstruction::Apply {
            path: VdTheoremPath::SquareNonnegative,
            is_real_coercion,
        };
        let hypothesis = self
            .hypothesis_constructor
            .construct_new_hypothesis(prop, construction);
        Ok(AltSome(hypothesis))
    }
}
