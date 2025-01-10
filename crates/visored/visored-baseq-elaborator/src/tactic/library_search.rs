use super::*;
use crate::{
    coercion::VdBsqCoercionOutcome, expr::VdBsqExprFldData,
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
            return AltNothing;
        }
    };
}

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub(crate) fn library_search(&mut self, prop: VdBsqExprFld<'sess>) -> Mhr<'sess> {
        self.with_call(VdBsqTacticCall::LibrarySearch, |slf| {
            slf.library_search_inner(prop)
        })
    }

    fn library_search_inner(&mut self, prop: VdBsqExprFld<'sess>) -> Mhr<'sess> {
        self.square_nonnegative(prop)?;
        AltNothing
    }

    fn square_nonnegative(&mut self, prop: VdBsqExprFld<'sess>) -> Mhr<'sess> {
        use husky_print_utils::*;
        let VdBsqExprFldData::ChainingSeparatedList {
            leader,
            followers,
            joined_signature: None,
        } = prop.data()
        else {
            return AltNothing;
        };
        assert!(followers.len() == 1);
        let (ge, rhs) = followers[0];
        let VdMirFunc::NormalBaseSeparator(ge) = ge else {
            unreachable!()
        };
        require!(ge.opr() == VdMirBaseSeparator::GE);
        require!(rhs.is_zero());
        let VdBsqExprFldData::Application {
            function: pow,
            arguments: pow_args,
        } = leader.data()
        else {
            unreachable!()
        };
        let VdMirFunc::Power(pow) = pow else {
            return AltNothing;
        };
        require!(pow_args[1].eqs_nat128(2));
        let Some(is_real_coercion) = pow_args[0].is_real(self).coercion() else {
            return AltNothing;
        };
        let construction = VdBsqHypothesisConstruction::Apply {
            path: VdTheoremPath::SquareNonnegative,
            is_real_coercion,
        };
        let hypothesis = self
            .hypothesis_constructor
            .construct_new_hypothesis(prop, construction);
        AltJustOk(Ok(hypothesis))
    }
}
