use super::*;
use crate::term::{
    builder::sum::VdBsqSumBuilder, comnum::product::VdBsqProductStem, litnum::VdBsqLitnumTerm,
};
use crate::term::{comnum::sum::VdBsqSumTerm, num::VdBsqNumTerm};
use elabm::Pure;
use miracle::{error::MiracleAltMaybeResult, foldm::foldm};
use product::foldm_product;
use std::marker::PhantomData;
use visored_baseq_elaborator_macros::unify_elabm;

pub(super) fn foldm_sum<'a, 'db, 'sess>(
    terms: &'a [(VdBsqProductStem<'sess>, VdBsqLitnumTerm<'sess>)],
    builder: VdBsqSumBuilder<'sess>,
) -> impl ElabM<'db, 'sess, VdBsqSumBuilder<'sess>> + 'a
where
    'db: 'sess,
{
    foldm(
        builder,
        terms.iter().copied(),
        |elaborator, builder, term, heuristic| {
            foldm_sum_step(builder, term).eval(elaborator, heuristic)
        },
    )
}

fn foldm_sum_step<'db, 'sess>(
    mut sum_builder: VdBsqSumBuilder<'sess>,
    (term, litnum0): (VdBsqProductStem<'sess>, VdBsqLitnumTerm<'sess>),
) -> impl ElabM<'db, 'sess, VdBsqSumBuilder<'sess>>
where
    'db: 'sess,
{
    #[unify_elabm]
    match term {
        VdBsqProductStem::Atom(atom) => {
            sum_builder.add_monomial(VdBsqProductStem::Atom(atom), litnum0);
            Pure(sum_builder)
        }
        VdBsqProductStem::NonTrivial(base) => {
            let db = elaborator.floater_db();
            foldm_product(base.exponentials()).map(|elaborator, expansion| {
                let mut sum_builder = sum_builder.clone();
                for (litnum, exponentials) in expansion {
                    sum_builder.add_general_product(
                        litnum0.mul(litnum, db),
                        VdBsqProductStem::from_parts(exponentials, db),
                    );
                }
                sum_builder
            })
        }
    }
}
