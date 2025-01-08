use super::*;
use crate::term::{
    inum::{
        VdBsqExponentialParts, VdBsqExponentialPowers, VdBsqExponentialPowersRef,
        VdBsqNonProductNumTerm, VdBsqNonSumInumTerm,
    },
    litnum::VdBsqLitNumTerm,
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
    type State = Option<Vec<(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)>>;

    type Item = (VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>);

    type Output = MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>;

    fn foldm_step(
        elaborator: &mut VdBsqElaboratorInner<'db, 'sess>,
        expansion: Option<Vec<(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)>>,
        (base, exponent): (VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>),
        f: &impl Fn(
            &mut VdBsqElaboratorInner<'db, 'sess>,
            Option<Vec<(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)>>,
        )
            -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>,
    ) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>> {
        let db = elaborator.floater_db();
        let config = elaborator.session().config().tactic().comm_ring();
        let product_expansion_limit = config.product_expansion_limit();
        let exponential_expansion_limit = config.exponential_expansion_limit();
        // returns error if the expansion exceeds comm ring product expansion limit
        let merge = |factor_expansion: &[(
            VdBsqLitNumTerm<'sess>,
            VdBsqExponentialParts<'sess>,
        )]|
         -> Result<Vec<(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)>, ()> {
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
             factor_expansion: &[(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)]| {
                let Ok(expansion) = merge(factor_expansion) else {
                    return AltNothing;
                };
                f(elaborator, Some(expansion))
            };
        elaborator.exec_batch(&[
            &|elaborator| g(elaborator, &[(1.into(), vec![(base, exponent)])]),
            &|elaborator| {
                let VdBsqNumTerm::Rnum(VdBsqLitNumTerm::Int128(exponent)) = exponent else {
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
                        &sum.nonzero_constant_term()
                            .map(|rnum| (rnum, vec![]))
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
                            as &[(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)],
                    )
                } else {
                    use combinatorics::try_multinomial_expansion;

                    let VdBsqNonProductNumTerm::SumInum(sum) = base else {
                        return AltNothing;
                    };
                    use husky_print_utils::p;
                    match sum.nonzero_constant_term() {
                        Some(_) => {
                            let n_summands_in_factor = sum.monomials().len() + 1;
                            let max_size = product_expansion_limit
                                / expansion.as_ref().map(|exp| exp.len()).unwrap_or(1);
                            match try_multinomial_expansion(
                                n_summands_in_factor as i128,
                                exponent,
                                max_size,
                            ) {
                                Ok(coefficients) => {
                                    let mut factor_expansion: Vec<(
                                        VdBsqLitNumTerm<'sess>,
                                        VdBsqExponentialParts<'sess>,
                                    )> = vec![];
                                    for (coeff, indices) in coefficients {
                                        let mut cumulative_coeff: VdBsqLitNumTerm = coeff.into();
                                        let mut exponential_parts: VdBsqExponentialParts<'sess> =
                                            vec![];
                                        for (i, index) in indices.into_iter().enumerate() {
                                            if index == 0 {
                                                continue;
                                            }
                                            if i == 0 {
                                                cumulative_coeff.mul_assign(
                                                    sum.constant_term().pow128(index, db).into(),
                                                    db,
                                                )
                                            } else {
                                                let (summand, coeff) =
                                                    sum.monomials().data()[(i - 1) as usize];
                                                cumulative_coeff
                                                    .mul_assign(coeff.pow128(index, db).into(), db);
                                                match summand {
                                                    VdBsqNonSumInumTerm::Atom(term) => {
                                                        let part = (term.into(), index.into());
                                                        exponential_parts.push(part);
                                                    }
                                                    VdBsqNonSumInumTerm::Product(base) => {
                                                        for &(base, exponent) in base.exponentials()
                                                        {
                                                            let part = (
                                                                base.into(),
                                                                exponent.mul128(index, db).into(),
                                                            );
                                                            exponential_parts.push(part);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        factor_expansion.push((coeff.into(), exponential_parts));
                                    }
                                    g(elaborator, &factor_expansion)
                                }
                                Err(_) => AltNothing,
                            }
                        }
                        None => {
                            let n_summands_in_factor = sum.monomials().len();
                            let max_size = product_expansion_limit
                                / expansion.as_ref().map(|exp| exp.len()).unwrap_or(1);
                            match try_multinomial_expansion(
                                n_summands_in_factor as i128,
                                exponent,
                                max_size,
                            ) {
                                Ok(coefficients) => {
                                    let mut factor_expansion: Vec<(
                                        VdBsqLitNumTerm<'sess>,
                                        VdBsqExponentialParts<'sess>,
                                    )> = vec![];
                                    for (coeff, indices) in coefficients {
                                        let mut cumulative_coeff: VdBsqLitNumTerm = coeff.into();
                                        let mut exponential_parts: VdBsqExponentialParts<'sess> =
                                            vec![];
                                        for (i, index) in indices.into_iter().enumerate() {
                                            if index == 0 {
                                                continue;
                                            }
                                            let (summand, coeff) = sum.monomials().data()[i];
                                            cumulative_coeff
                                                .mul_assign(coeff.pow128(index, db).into(), db);
                                            match summand {
                                                VdBsqNonSumInumTerm::Atom(term) => {
                                                    let part = (term.into(), index.into());
                                                    exponential_parts.push(part);
                                                }
                                                VdBsqNonSumInumTerm::Product(base) => {
                                                    for &(base, exponent) in base.exponentials() {
                                                        let part = (
                                                            base.into(),
                                                            exponent.mul128(index, db).into(),
                                                        );
                                                        exponential_parts.push(part);
                                                    }
                                                }
                                            }
                                        }
                                        factor_expansion.push((coeff.into(), exponential_parts));
                                    }
                                    g(elaborator, &factor_expansion)
                                }
                                Err(_) => AltNothing,
                            }
                        }
                    }
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
        Option<Vec<(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)>>,
    ) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>,
) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>> {
    engine.foldm::<_, _, _, _>(
        &None,
        exponentials.iter().copied(),
        &[
            multiply_without_expanding as FnType<'db, 'sess>,
            multiply_with_expanding as FnType<'db, 'sess>,
        ],
        &|_, _| todo!(),
    )
}

type FnType<'db, 'sess> =
    for<'a, 'b, 'c> fn(
        &'a mut VdBsqElaboratorInner<'b, 'c>,
        &Option<Vec<(VdBsqLitNumTerm<'c>, VdBsqExponentialParts<'c>)>>,
        &(VdBsqNonProductNumTerm<'c>, VdBsqNumTerm<'c>),
    ) -> Option<Vec<(VdBsqLitNumTerm<'c>, VdBsqExponentialParts<'c>)>>;

fn multiply_without_expanding<'db, 'sess>(
    engine: &mut VdBsqElaboratorInner<'db, 'sess>,
    expansion: &Option<Vec<(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)>>,
    (base, exponent): &(VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>),
) -> Option<Vec<(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)>> {
    todo!()
}

fn multiply_with_expanding<'db, 'sess>(
    engine: &mut VdBsqElaboratorInner<'db, 'sess>,
    expansion: &Option<Vec<(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)>>,
    (base, exponent): &(VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>),
) -> Option<Vec<(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)>> {
    todo!()
}
