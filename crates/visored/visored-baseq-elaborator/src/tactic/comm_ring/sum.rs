use super::*;
use crate::term::{
    builder::sum::VdBsqSumBuilder, comnum::product::VdBsqProductComnumTermBase,
    litnum::VdBsqLitnumTerm,
};
use crate::term::{
    comnum::{sum::VdBsqSumComnumTerm, VdBsqNonSumComnumTerm},
    num::VdBsqNumTerm,
};
use miracle::error::MiracleAltMaybeResult;
use std::marker::PhantomData;

pub(super) fn foldm_sum<'db, 'sess>(
    engine: &mut VdBsqElaboratorInner<'db, 'sess>,
    terms: &[(VdBsqNonSumComnumTerm<'sess>, VdBsqLitnumTerm<'sess>)],
    builder: VdBsqSumBuilder<'sess>,
    f: &impl Fn(
        &mut VdBsqElaboratorInner<'db, 'sess>,
        VdBsqSumBuilder<'sess>,
    ) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>,
) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>> {
    engine.foldm(builder, terms.iter().copied(), f, &foldm_sum_step)
}

fn foldm_sum_step<'db, 'sess>(
    elaborator: &mut VdBsqElaboratorInner<'db, 'sess>,
    mut sum_builder: VdBsqSumBuilder<'sess>,
    (term, litnum0): (VdBsqNonSumComnumTerm<'sess>, VdBsqLitnumTerm<'sess>),
    f: &dyn Fn(
        &mut VdBsqElaboratorInner<'db, 'sess>,
        VdBsqSumBuilder<'sess>,
    )
        -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>,
) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>> {
    let db = elaborator.floater_db();
    match term {
        VdBsqNonSumComnumTerm::Atom(atom) => {
            sum_builder.add_litnum_times_atom(litnum0, atom);
            f(elaborator, sum_builder)
        }
        VdBsqNonSumComnumTerm::Product(base) => {
            foldm_product(elaborator, base.exponentials(), &|elaborator, expansion| {
                let mut sum_builder = sum_builder.clone();
                for (litnum, exponentials) in expansion {
                    sum_builder.add_general_product(
                        litnum0.mul(litnum, db),
                        VdBsqProductComnumTermBase::from_parts(exponentials, db),
                    );
                }
                f(elaborator, sum_builder)
            })
        }
    }
}
