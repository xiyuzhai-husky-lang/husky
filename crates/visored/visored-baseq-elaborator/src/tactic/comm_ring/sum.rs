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
use monadic_fold::engine::{IsMonadicFoldEngineScheme, IsMonadicFoldEngineSchemeFull as _};
use std::marker::PhantomData;

struct Scheme<'db, 'sess>(PhantomData<&'db ()>, PhantomData<&'sess ()>);

impl<'db, 'sess> IsMonadicFoldEngineScheme for Scheme<'db, 'sess>
where
    'db: 'sess,
{
    type Engine = VdBsqElaboratorInner<'db, 'sess>;

    type State = VdBsqSumBuilder<'sess>;

    type Item = (VdBsqNonSumComnumTerm<'sess>, VdBsqLitnumTerm<'sess>);

    type Output = MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>;

    /// add term times litnum0 to partial sum (sum_builder)
    fn foldm_step(
        elaborator: &mut Self::Engine,
        mut sum_builder: VdBsqSumBuilder<'sess>,
        (term, litnum0): (VdBsqNonSumComnumTerm<'sess>, VdBsqLitnumTerm<'sess>),
        f: &impl Fn(&mut VdBsqElaboratorInner<'db, 'sess>, VdBsqSumBuilder<'sess>) -> Self::Output,
    ) -> Self::Output {
        let db = elaborator.floater_db();
        match term {
            VdBsqNonSumComnumTerm::Atom(atom) => {
                sum_builder.add_litnum_times_atom(litnum0, atom);
                f(elaborator, sum_builder)
            }
            VdBsqNonSumComnumTerm::Product(base) => {
                fold_product(elaborator, base.exponentials(), &|elaborator, expansion| {
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
}

pub(super) fn fold_sum<'db, 'sess>(
    engine: &mut VdBsqElaboratorInner<'db, 'sess>,
    terms: &[(VdBsqNonSumComnumTerm<'sess>, VdBsqLitnumTerm<'sess>)],
    builder: VdBsqSumBuilder<'sess>,
    f: &impl Fn(
        &mut VdBsqElaboratorInner<'db, 'sess>,
        VdBsqSumBuilder<'sess>,
    ) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>,
) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>> {
    Scheme::foldm(engine, builder, terms.iter().copied(), f)
}
