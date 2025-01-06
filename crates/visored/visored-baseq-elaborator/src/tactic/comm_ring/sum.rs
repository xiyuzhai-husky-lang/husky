use super::*;
use crate::term::{
    builder::sum::VdBsqSumBuilder, inum::product::VdBsqProductInumTermBase, rnum::VdBsqRnumTerm,
};
use crate::term::{
    inum::{sum::VdBsqSumInumTerm, VdBsqNonSumInumTerm},
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

    type Item = (VdBsqNonSumInumTerm<'sess>, VdBsqRnumTerm);

    type Output = MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>;

    fn foldm_step(
        elaborator: &mut Self::Engine,
        builder: VdBsqSumBuilder<'sess>,
        (term, rnum): (VdBsqNonSumInumTerm<'sess>, VdBsqRnumTerm),
        f: &impl Fn(&mut VdBsqElaboratorInner<'db, 'sess>, VdBsqSumBuilder<'sess>) -> Self::Output,
    ) -> Self::Output {
        match term {
            VdBsqNonSumInumTerm::Atom(vd_bsq_atom_inum_term) => todo!(),
            VdBsqNonSumInumTerm::Product(vd_bsq_product_inum_term_base) => fold_product(
                elaborator,
                vd_bsq_product_inum_term_base.exponentials(),
                &|elaborator, expansion| {
                    let mut sum_builder = builder.clone();
                    for (rnum, exponentials) in
                        expansion.expect("expansion shouldn't be None after folding")
                    {
                        sum_builder.add_general_product(
                            rnum,
                            VdBsqProductInumTermBase::from_parts(
                                exponentials,
                                elaborator.floater_db(),
                            ),
                        );
                    }
                    f(elaborator, sum_builder)
                },
            ),
        }
    }
}

pub(super) fn fold_sum<'db, 'sess>(
    engine: &mut VdBsqElaboratorInner<'db, 'sess>,
    terms: &[(VdBsqNonSumInumTerm<'sess>, VdBsqRnumTerm)],
    builder: VdBsqSumBuilder<'sess>,
    f: &impl Fn(
        &mut VdBsqElaboratorInner<'db, 'sess>,
        VdBsqSumBuilder<'sess>,
    ) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>,
) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>> {
    Scheme::foldm(engine, builder, terms.iter().copied(), f)
}
