use super::*;
use crate::maneuver::litnum_rewrite::litnum_rewritem;
use crate::{
    coercion::VdBsqCoercionOutcome, expr::VdBsqExprFldData,
    hypothesis::construction::VdBsqHypothesisConstruction,
};
use alt_option::*;
use foundations::opr::separator::relation::comparison::VdBsqBoundOpr;
use term::{litnum::VdBsqLitnumTerm, prop::VdBsqPropTerm, VdBsqTerm};
use visored_baseq_elaborator_macros::unify_elabm;
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

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub(crate) fn litnum_estimate(&mut self, prop: VdBsqExprFld<'sess>) -> Mhr<'sess> {
        self.with_call(VdBsqTacticCall::LitnumEstimate, |slf| {
            slf.litnum_estimate_inner(prop)
        })
    }

    fn litnum_estimate_inner(&mut self, prop: VdBsqExprFld<'sess>) -> Mhr<'sess> {
        let VdBsqExprFldData::ChainingSeparatedList {
            leader,
            ref followers,
            joined_signature,
        } = *prop.data()
        else {
            return AltNothing;
        };
        if followers.len() != 1 {
            return AltNothing;
        }
        let relationship = match followers[0].0 {
            VdMirFunc::NormalBaseSeparator(signature) => {
                VdBsqBoundOpr::from_mir_base_separator(signature.opr())?
            }
            _ => return AltNothing,
        };
        let VdBsqTerm::Litnum(rhs) = followers[0].1.term() else {
            return AltNothing;
        };
        try_all(self, leader, relationship, rhs)
    }
}

fn try_all<'db, 'sess>(
    elr: &mut VdBsqElaboratorInner<'db, 'sess>,
    leader: VdBsqExprFld<'sess>,
    comp: VdBsqBoundOpr,
    rhs: VdBsqLitnumTerm<'sess>,
) -> Mhr<'sess> {
    try_one_shot(elr, leader, comp, rhs)?;
    AltNothing
}

fn try_one_shot<'db, 'sess>(
    elr: &mut VdBsqElaboratorInner<'db, 'sess>,
    leader: VdBsqExprFld<'sess>,
    comp: VdBsqBoundOpr,
    rhs: VdBsqLitnumTerm<'sess>,
) -> Mhr<'sess> {
    // elr.hypothesis_constructor
    //     .stack()
    //     .get_active_litnum_inequality(expr, db);
    todo!()
}
