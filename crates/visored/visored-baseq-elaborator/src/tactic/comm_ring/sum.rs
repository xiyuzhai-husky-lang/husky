use super::*;
use crate::term::{
    builder::sum::VdBsqSumBuilder, comnum::product::VdBsqProductBase, litnum::VdBsqLitnumTerm,
};
use crate::term::{comnum::sum::VdBsqSumTerm, num::VdBsqNumTerm};
use elabm::Pure;
use miracle::{error::MiracleAltMaybeResult, foldm::foldm};
use product::foldm_product;
use std::marker::PhantomData;
use visored_baseq_elaborator_macros::unify_elabm;

pub(super) fn foldm_sum<'a, 'db, 'sess>(
    terms: &'a [(VdBsqProductBase<'sess>, VdBsqLitnumTerm<'sess>)],
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
    (term, litnum0): (VdBsqProductBase<'sess>, VdBsqLitnumTerm<'sess>),
) -> impl ElabM<'db, 'sess, VdBsqSumBuilder<'sess>>
where
    'db: 'sess,
{
    #[unify_elabm]
    match term {
        VdBsqProductBase::Atom(atom) => {
            sum_builder.add_monomial(VdBsqProductBase::Atom(atom), litnum0);
            Pure(sum_builder)
        }
        VdBsqProductBase::Sum(sum) => {
            let db = elaborator.floater_db();
            sum_builder.add_litnum(litnum0.mul(sum.constant_term(), db));
            for &(base, litnum) in sum.monomials() {
                sum_builder.add_monomial(base, litnum0.mul(litnum, db));
            }
            Pure(sum_builder)
        }
        VdBsqProductBase::NonTrivial(base) => {
            let db = elaborator.floater_db();
            foldm_product(base.exponentials()).map(|elaborator, expansion| {
                let mut sum_builder = sum_builder.clone();
                for (litnum, exponentials) in expansion {
                    sum_builder.add_general_product(
                        litnum0.mul(litnum, db),
                        VdBsqProductBase::from_parts(exponentials, db),
                    );
                }
                sum_builder
            })
        }
    }
}
