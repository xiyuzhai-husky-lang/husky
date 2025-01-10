mod product;
mod sum;

use self::product::foldm_product;
use self::sum::foldm_sum;
use super::*;
use crate::{
    hypothesis::construction::VdBsqHypothesisConstruction,
    term::{
        builder::sum::VdBsqSumBuilder,
        comnum::VdBsqComnumTerm,
        num::VdBsqNumTerm,
        prop::{
            num_relationship::{
                VdBsqNumRelationshipPropTermData, VdBsqNumRelationshipPropTermKind,
            },
            VdBsqPropTerm,
        },
        VdBsqTerm,
    },
};
use alt_maybe_result::*;
use alt_option::*;

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub(crate) fn comm_ring(&mut self, prop: VdBsqExprFld<'sess>) -> Mhr<'sess> {
        self.with_call(VdBsqTacticCall::CommRing, |slf| slf.comm_ring_inner(prop))
    }

    fn comm_ring_inner(&mut self, prop: VdBsqExprFld<'sess>) -> Mhr<'sess> {
        let VdBsqTerm::Prop(VdBsqPropTerm::NumRelationship(num_relationship)) = prop.term() else {
            return AltNothing;
        };
        let VdBsqNumRelationshipPropTermData {
            kind,
            lhs_minus_rhs: VdBsqNumTerm::Comnum(VdBsqComnumTerm::Sum(term)),
        } = *num_relationship.data()
        else {
            return AltNothing;
        };
        let mut builder = VdBsqSumBuilder::new(self.floater_db());
        builder.add_litnum(term.constant_term());
        foldm_sum(term.monomials(), builder).eval(self, &|slf, builder| {
            let term = builder.finish();
            let VdBsqNumTerm::Litnum(litnum) = term else {
                return AltNothing;
            };
            match litnum.compare_with_zero(kind) {
                true => {
                    let hypothesis = slf
                        .hypothesis_constructor
                        .construct_new_hypothesis(prop, VdBsqHypothesisConstruction::CommRing);
                    AltJustOk(Ok(hypothesis))
                }
                false => todo!("report error, litnum = {:?}, kind = {:?}", litnum, kind),
            }
        })
    }
}
