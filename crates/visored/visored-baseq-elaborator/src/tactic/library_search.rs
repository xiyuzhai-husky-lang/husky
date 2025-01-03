use super::*;
use crate::{expr::VdMirExprFldData, hypothesis::construction::VdBaseqHypothesisConstruction};
use alt_option::*;
use visored_entity_path::{
    path::{
        set::{VdPreludeSetPath, VdSetPath},
        VdItemPath,
    },
    theorem::VdTheoremPath,
};
use visored_mir_expr::expr::application::VdMirFunc;
use visored_opr::separator::VdBaseSeparator;
use visored_term::term::VdTerm;

macro_rules! require {
    ($condition:expr) => {
        if !$condition {
            return Ok(AltNone);
        }
    };
}

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
        require!(ge.opr() == VdBaseSeparator::GE);
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
        match *pow_args[0].ty() {
            VdTerm::ItemPath(path) => match path.item_path() {
                VdItemPath::Category(vd_category_path) => todo!(),
                VdItemPath::Set(path) => match path {
                    VdSetPath::Prelude(path) => match path {
                        VdPreludeSetPath::NaturalNumber
                        | VdPreludeSetPath::RationalNumber
                        | VdPreludeSetPath::Integer
                        | VdPreludeSetPath::RealNumber => {
                            let construction = VdBaseqHypothesisConstruction::Apply {
                                path: VdTheoremPath::SquareNonnegative,
                            };
                            let hypothesis = self
                                .hypothesis_constructor
                                .construct_new_hypothesis(prop, construction);
                            Ok(AltSome(hypothesis))
                        }
                        VdPreludeSetPath::ComplexNumber => Ok(AltNone),
                    },
                },
                VdItemPath::Function(vd_function_path) => todo!(),
                VdItemPath::Trait(vd_trait_path) => todo!(),
                VdItemPath::TraitItem(vd_trait_item_path) => todo!(),
            },
            _ => todo!(),
        }
    }
}
