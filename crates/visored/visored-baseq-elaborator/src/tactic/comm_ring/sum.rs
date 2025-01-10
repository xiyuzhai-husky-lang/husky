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
use monad::Pure;
use product::foldm_product;
use std::marker::PhantomData;

pub(super) fn foldm_sum<'a, 'db, 'sess>(
    terms: &'a [(VdBsqNonSumComnumTerm<'sess>, VdBsqLitnumTerm<'sess>)],
    builder: VdBsqSumBuilder<'sess>,
) -> impl ElabM<'db, 'sess, VdBsqSumBuilder<'sess>> + 'a
where
    'db: 'sess,
{
    HasMiracleFull::foldm(
        builder,
        terms.iter().copied(),
        &|elaborator, builder, term, heuristic| {
            foldm_sum_step(builder, term).eval(elaborator, heuristic)
        },
    )
}

fn foldm_sum_step<'db, 'sess>(
    mut sum_builder: VdBsqSumBuilder<'sess>,
    (term, litnum0): (VdBsqNonSumComnumTerm<'sess>, VdBsqLitnumTerm<'sess>),
) -> impl ElabM<'db, 'sess, VdBsqSumBuilder<'sess>>
where
    'db: 'sess,
{
    move |elaborator: &mut VdBsqElaboratorInner<'db, 'sess>,
          heuristic: &Heuristic<'_, 'db, 'sess, VdBsqSumBuilder<'sess>>| {
        let db = elaborator.floater_db();
        match term {
            VdBsqNonSumComnumTerm::Atom(atom) => {
                sum_builder.add_litnum_times_atom(litnum0, atom);
                Pure(sum_builder).eval(elaborator, heuristic)
            }
            VdBsqNonSumComnumTerm::Product(base) => foldm_product(base.exponentials())
                .map(|elaborator, expansion| {
                    let mut sum_builder = sum_builder.clone();
                    for (litnum, exponentials) in expansion {
                        sum_builder.add_general_product(
                            litnum0.mul(litnum, db),
                            VdBsqProductComnumTermBase::from_parts(exponentials, db),
                        );
                    }
                    sum_builder
                })
                .eval(elaborator, heuristic),
        }
    }
}
