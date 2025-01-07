use super::*;
use crate::term::{
    inum::{
        VdBsqExponentialParts, VdBsqExponentialPowers, VdBsqExponentialPowersRef,
        VdBsqNonProductNumTerm, VdBsqNonSumInumTerm,
    },
    rnum::VdBsqRnumTerm,
};
use itertools::Itertools;
use miracle::error::MiracleAltMaybeResult;
use monadic_fold::engine::IsMonadicFoldEngineScheme;
use monadic_fold::engine::IsMonadicFoldEngineSchemeFull;
use std::marker::PhantomData;

struct Scheme<'db, 'sess>(PhantomData<(&'db (), &'sess ())>);

impl<'db, 'sess> IsMonadicFoldEngineScheme for Scheme<'db, 'sess>
where
    'db: 'sess,
{
    type Engine = VdBsqElaboratorInner<'db, 'sess>;

    /// Initial state is set to None.
    type State = Option<Vec<(VdBsqRnumTerm, VdBsqExponentialParts<'sess>)>>;

    type Item = (VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>);

    type Output = MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>;

    fn foldm_step(
        elaborator: &mut VdBsqElaboratorInner<'db, 'sess>,
        expansion: Option<Vec<(VdBsqRnumTerm, VdBsqExponentialParts<'sess>)>>,
        (base, exponent): (VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>),
        f: &impl Fn(
            &mut VdBsqElaboratorInner<'db, 'sess>,
            Option<Vec<(VdBsqRnumTerm, VdBsqExponentialParts<'sess>)>>,
        )
            -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>,
    ) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>> {
        let db = elaborator.floater_db();
        let config = elaborator.session().config().tactic().comm_ring();
        let product_expansion_limit = config.product_expansion_limit();
        let exponential_expansion_limit = config.exponential_expansion_limit();
        // returns error if the expansion exceeds comm ring product expansion limit
        let merge = |factor_expansion: &[(
            VdBsqRnumTerm,
            VdBsqExponentialParts<'sess>,
        )]|
         -> Result<Vec<(VdBsqRnumTerm, VdBsqExponentialParts<'sess>)>, ()> {
            match expansion {
                Some(ref expansion) => {
                    if expansion.len() * factor_expansion.len() > product_expansion_limit {
                        return Err(());
                    }
                    Ok(expansion
                        .iter()
                        .cartesian_product(factor_expansion)
                        .map(|(&(rnum0, ref exponentials0), &(rnum1, ref exponentials1))| {
                            (
                                rnum0.mul(rnum1, db),
                                exponentials0.into_iter().chain(exponentials1).copied().collect(),
                            )
                        })
                        .collect())
                },
                None => {
                    if factor_expansion.len() > product_expansion_limit {
                        return Err(());
                    }
                    Ok(factor_expansion
                        .iter()
                        .map(|&(rnum, ref exponentials)| (rnum, exponentials.to_vec()))
                        .collect())
                }
            }
        };
        let g =
            |elaborator: &mut VdBsqElaboratorInner<'db, 'sess>,
             factor_expansion: &[(VdBsqRnumTerm, VdBsqExponentialParts<'sess>)]| {
                let Ok(expansion) = merge(factor_expansion) else {
                    return AltNothing;
                };
                use husky_print_utils::p;
                p!(expansion);
                f(elaborator, Some(expansion))
            };
        elaborator.exec_batch(&[
            &|elaborator| g(elaborator, &[(1.into(), vec![(base, exponent)])]),
            &|elaborator| {
                let VdBsqNumTerm::Rnum(VdBsqRnumTerm::Int128(exponent)) = exponent else {
                    return AltNothing;
                };
                if exponent > exponential_expansion_limit as i128 {
                    return AltNothing;
                }
                debug_assert!(exponent > 0);
                if exponent == 1 {
                    let VdBsqNonProductNumTerm::SumInum(sum) = base else {
                        return AltNothing;
                    };
                    g(
                        elaborator,
                        &[(sum.constant_term(), vec![])]
                            .into_iter()
                            .chain(sum.monomials().iter().map(|&(monomial, coeff)| {
                                (
                                    coeff,
                                    match monomial {
                                        VdBsqNonSumInumTerm::Atom(atom) => {
                                            vec![(atom.into(), 1.into())]
                                        }
                                        VdBsqNonSumInumTerm::Product(base) => {
                                            base.exponentials().to_vec()
                                        }
                                    },
                                )
                            }))
                            .collect::<Vec<_>>()
                            as &[(VdBsqRnumTerm, VdBsqExponentialParts<'sess>)],
                    )
                } else {
                    let VdBsqNonProductNumTerm::SumInum(sum) = base else {
                        return AltNothing;
                    };
                    use husky_print_utils::p;
                    p!(sum);
                    // todo!()
                    AltNothing
                }
            },
        ])
    }
}

pub fn fold_product<'db, 'sess>(
    engine: &mut VdBsqElaboratorInner<'db, 'sess>,
    exponentials: &[(VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>)],
    f: &impl Fn(
        &mut VdBsqElaboratorInner<'db, 'sess>,
        Option<Vec<(VdBsqRnumTerm, VdBsqExponentialParts<'sess>)>>,
    ) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>,
) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>> {
    Scheme::foldm(engine, None, exponentials.iter().copied(), f)
}
