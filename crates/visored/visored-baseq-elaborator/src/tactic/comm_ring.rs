mod product;
mod sum;

use self::product::fold_product;
use self::sum::fold_sum;
use super::*;
use crate::{
    hypothesis::construction::VdBsqHypothesisConstruction,
    term::{
        builder::sum::VdBsqSumBuilder,
        inum::VdBsqInumTerm,
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
    pub(crate) fn comm_ring(
        &mut self,
        prop: VdMirExprFld<'sess>,
    ) -> VdBsqHypothesisResult<'sess, AltOption<VdBsqHypothesisIdx<'sess>>> {
        let VdBsqTerm::Prop(VdBsqPropTerm::NumRelationship(num_relationship)) = prop.term() else {
            return Ok(AltNone);
        };
        let VdBsqNumRelationshipPropTermData {
            kind,
            lhs_minus_rhs: VdBsqNumTerm::Inum(VdBsqInumTerm::Sum(term)),
        } = *num_relationship.data()
        else {
            return Ok(AltNone);
        };
        let config = self.session().config().tactic().comm_ring();
        match self.run_staged(config.stages(), config.max_heartbeats(), |slf| {
            let mut builder = VdBsqSumBuilder::new(slf.floater_db());
            builder.add_rnum(term.constant_term());
            fold_sum(slf, term.monomials(), builder, &|elaborator, builder| {
                let term = builder.finish();
                let VdBsqNumTerm::Rnum(rnum) = term else {
                    use husky_print_utils::p;
                    match term {
                        VdBsqNumTerm::Rnum(term) => (),
                        VdBsqNumTerm::Inum(term) => match term {
                            VdBsqInumTerm::Atom(term) => (),
                            VdBsqInumTerm::Sum(term) => {
                                if term.monomials().len() > 1 {
                                    use husky_print_utils::p;
                                    p!(term.monomials().data()[0] == term.monomials().data()[1]);
                                }
                                p!(term, term.monomials());
                            }
                            VdBsqInumTerm::Product(term, _) => (),
                        },
                    }
                    p!(term);
                    return AltNothing;
                };
                match rnum.compare_with_zero(kind) {
                    true => {
                        let hypothesis = elaborator
                            .hypothesis_constructor
                            .construct_new_hypothesis(prop, VdBsqHypothesisConstruction::CommRing);
                        AltJustOk(Ok(hypothesis))
                    }
                    false => todo!("report error, rnum = {:?}, kind = {:?}", rnum, kind),
                }
            })
        }) {
            AltJustOk(result) => match result {
                Ok(hypothesis) => Ok(AltSome(hypothesis)),
                Err(contradiction) => Err(contradiction),
            },
            AltJustErr(_) | AltNothing => Ok(AltNone),
        }
    }
}
