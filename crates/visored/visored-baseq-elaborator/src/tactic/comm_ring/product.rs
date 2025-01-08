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

pub fn fold_product<'db, 'sess>(
    engine: &mut VdBsqElaboratorInner<'db, 'sess>,
    exponentials: &[(VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>)],
    f: &impl Fn(
        &mut VdBsqElaboratorInner<'db, 'sess>,
        Option<Vec<(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)>>,
    ) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>,
) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>> {
    engine.foldm::<_, _, _, _>(
        None,
        exponentials.iter().copied(),
        &[
            multiply_without_expanding as FnType,
            multiply_with_expanding as FnType,
        ],
        f,
    )
}

type State<'sess> = Option<Vec<(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)>>;
type Item<'sess> = (VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>);
type FnType<'db, 'sess> =
    fn(&mut VdBsqElaboratorInner<'db, 'sess>, &State<'sess>, &Item<'sess>) -> Option<State<'sess>>;

fn multiply_without_expanding<'db, 'sess>(
    elaborator: &mut VdBsqElaboratorInner<'db, 'sess>,
    expansion: &State<'sess>,
    &(base, exponent): &Item<'sess>,
) -> Option<State<'sess>> {
    let factor_expansion = &[(1.into(), vec![(base, exponent)])];
    multiply_aux(elaborator, expansion, factor_expansion)
}

fn multiply_with_expanding<'db, 'sess>(
    elaborator: &mut VdBsqElaboratorInner<'db, 'sess>,
    expansion: &State<'sess>,
    &(base, exponent): &Item<'sess>,
) -> Option<State<'sess>> {
    let db = elaborator.floater_db();
    let config = elaborator.session().config().tactic().comm_ring();
    let product_expansion_limit = config.product_expansion_limit();
    let exponential_expansion_limit = config.exponential_expansion_limit();
    let VdBsqNumTerm::Rnum(VdBsqLitNumTerm::Int128(exponent)) = exponent else {
        return None;
    };
    if exponent > exponential_expansion_limit as i128 {
        return None;
    }
    debug_assert!(exponent > 0);
    let VdBsqNonProductNumTerm::SumInum(sum) = base else {
        return None;
    };
    let factor_expansion = if exponent == 1 {
        sum.nonzero_constant_term()
            .map(|rnum| (rnum, vec![]))
            .into_iter()
            .chain(sum.monomials().iter().map(|&(monomial, coeff)| {
                (
                    coeff,
                    match monomial {
                        VdBsqNonSumInumTerm::Atom(atom) => {
                            vec![(atom.into(), 1.into())]
                        }
                        VdBsqNonSumInumTerm::Product(base) => base.exponentials().to_vec(),
                    },
                )
            }))
            .collect::<Vec<_>>()
    } else {
        use combinatorics::try_multinomial_expansion;

        match sum.nonzero_constant_term() {
            Some(_) => {
                let n_summands_in_factor = sum.monomials().len() + 1;
                let max_size =
                    product_expansion_limit / expansion.as_ref().map(|exp| exp.len()).unwrap_or(1);
                match try_multinomial_expansion(n_summands_in_factor as i128, exponent, max_size) {
                    Ok(coefficients) => {
                        let mut factor_expansion: Vec<(
                            VdBsqLitNumTerm<'sess>,
                            VdBsqExponentialParts<'sess>,
                        )> = vec![];
                        for (coeff, indices) in coefficients {
                            let mut cumulative_coeff: VdBsqLitNumTerm = coeff.into();
                            let mut exponential_parts: VdBsqExponentialParts<'sess> = vec![];
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
                                    let (summand, coeff) = sum.monomials().data()[(i - 1) as usize];
                                    cumulative_coeff.mul_assign(coeff.pow128(index, db).into(), db);
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
                            }
                            factor_expansion.push((coeff.into(), exponential_parts));
                        }
                        factor_expansion
                    }
                    Err(_) => return None,
                }
            }
            None => {
                let n_summands_in_factor = sum.monomials().len();
                let max_size =
                    product_expansion_limit / expansion.as_ref().map(|exp| exp.len()).unwrap_or(1);
                match try_multinomial_expansion(n_summands_in_factor as i128, exponent, max_size) {
                    Ok(coefficients) => {
                        let mut factor_expansion: Vec<(
                            VdBsqLitNumTerm<'sess>,
                            VdBsqExponentialParts<'sess>,
                        )> = vec![];
                        for (coeff, indices) in coefficients {
                            let mut cumulative_coeff: VdBsqLitNumTerm = coeff.into();
                            let mut exponential_parts: VdBsqExponentialParts<'sess> = vec![];
                            for (i, index) in indices.into_iter().enumerate() {
                                if index == 0 {
                                    continue;
                                }
                                let (summand, coeff) = sum.monomials().data()[i];
                                cumulative_coeff.mul_assign(coeff.pow128(index, db).into(), db);
                                match summand {
                                    VdBsqNonSumInumTerm::Atom(term) => {
                                        let part = (term.into(), index.into());
                                        exponential_parts.push(part);
                                    }
                                    VdBsqNonSumInumTerm::Product(base) => {
                                        for &(base, exponent) in base.exponentials() {
                                            let part =
                                                (base.into(), exponent.mul128(index, db).into());
                                            exponential_parts.push(part);
                                        }
                                    }
                                }
                            }
                            factor_expansion.push((coeff.into(), exponential_parts));
                        }
                        factor_expansion
                    }
                    Err(_) => return None,
                }
            }
        }
    };
    multiply_aux(elaborator, expansion, &factor_expansion)
}

fn multiply_aux<'db, 'sess>(
    elaborator: &mut VdBsqElaboratorInner<'db, 'sess>,
    expansion: &State<'sess>,
    factor_expansion: &[(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)],
) -> Option<State<'sess>> {
    let db = elaborator.floater_db();
    let config = elaborator.session().config().tactic().comm_ring();
    let product_expansion_limit = config.product_expansion_limit();
    match expansion {
        Some(ref expansion) => {
            if expansion.len() * factor_expansion.len() > product_expansion_limit {
                return None;
            }
            Some(Some(
                expansion
                    .iter()
                    .cartesian_product(factor_expansion)
                    .map(
                        |(&(rnum0, ref exponentials0), &(rnum1, ref exponentials1))| {
                            (
                                rnum0.mul(rnum1, db),
                                exponentials0
                                    .into_iter()
                                    .chain(exponentials1)
                                    .copied()
                                    .collect(),
                            )
                        },
                    )
                    .collect(),
            ))
        }
        None => {
            if factor_expansion.len() > product_expansion_limit {
                return None;
            }
            Some(Some(
                factor_expansion
                    .iter()
                    .map(|&(rnum, ref exponentials)| (rnum, exponentials.to_vec()))
                    .collect(),
            ))
        }
    }
}
