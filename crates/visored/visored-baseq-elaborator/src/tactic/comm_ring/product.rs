use super::*;
use crate::term::{
    inum::{
        sum::VdBsqSumInumTerm, VdBsqExponentialParts, VdBsqExponentialPowers,
        VdBsqExponentialPowersRef, VdBsqNonProductNumTerm, VdBsqNonSumInumTerm,
    },
    litnum::VdBsqLitNumTerm,
};
use floated_sequential::db::FloaterDb;
use itertools::Itertools;
use miracle::error::MiracleAltMaybeResult;

pub fn fold_product<'db, 'sess>(
    engine: &mut VdBsqElaboratorInner<'db, 'sess>,
    exponentials: &[(VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>)],
    f: &impl Fn(
        &mut VdBsqElaboratorInner<'db, 'sess>,
        Vec<(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)>,
    ) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>,
) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>> {
    engine.multifold(
        vec![],
        exponentials.iter().copied(),
        &[
            multiply_without_expanding as FnType,
            multiply_with_expanding as FnType,
        ],
        f,
    )
}

type State<'sess> = Vec<(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)>;
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
        let max_size = product_expansion_limit / expansion.len().max(1);
        let has_constant_term = sum.nonzero_constant_term().is_some();
        multinomial_expansion(sum, exponent, max_size, db, has_constant_term)?
    };
    multiply_aux(elaborator, expansion, &factor_expansion)
}

fn multinomial_expansion<'db, 'sess>(
    sum: VdBsqSumInumTerm<'sess>,
    exponent: i128,
    max_size: usize,
    db: &'sess FloaterDb,
    has_constant_term: bool,
) -> Option<Vec<(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)>> {
    use combinatorics::try_multinomial_expansion_coefficients;

    let n_summands = if has_constant_term {
        sum.monomials().len() + 1
    } else {
        sum.monomials().len()
    };

    let coefficients =
        try_multinomial_expansion_coefficients(n_summands as i128, exponent, max_size)
            .map_err(|_| ())
            .ok()?;
    let mut factor_expansion = Vec::new();

    for (coeff, indices) in coefficients {
        let mut cumulative_coeff: VdBsqLitNumTerm = coeff.into();
        let mut exponential_parts = Vec::new();

        for (i, index) in indices.into_iter().enumerate() {
            if index == 0 {
                continue;
            }
            if has_constant_term && i == 0 {
                cumulative_coeff.mul_assign(sum.constant_term().pow128(index, db).into(), db);
                continue;
            }
            let monomial_idx = if has_constant_term { i - 1 } else { i };
            let (summand, coeff) = sum.monomials().data()[monomial_idx];
            cumulative_coeff.mul_assign(coeff.pow128(index, db).into(), db);
            match summand {
                VdBsqNonSumInumTerm::Atom(term) => {
                    exponential_parts.push((term.into(), index.into()));
                }
                VdBsqNonSumInumTerm::Product(base) => {
                    for &(base, exp) in base.exponentials() {
                        exponential_parts.push((base.into(), exp.mul128(index, db).into()));
                    }
                }
            }
        }
        factor_expansion.push((coeff.into(), exponential_parts));
    }

    Some(factor_expansion)
}

fn multiply_aux<'db, 'sess>(
    elaborator: &mut VdBsqElaboratorInner<'db, 'sess>,
    expansion: &State<'sess>,
    factor_expansion: &[(VdBsqLitNumTerm<'sess>, VdBsqExponentialParts<'sess>)],
) -> Option<State<'sess>> {
    let db = elaborator.floater_db();
    let config = elaborator.session().config().tactic().comm_ring();
    let product_expansion_limit = config.product_expansion_limit();
    match expansion.is_empty() {
        false => {
            if expansion.len() * factor_expansion.len() > product_expansion_limit {
                return None;
            }
            Some(
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
            )
        }
        true => {
            if factor_expansion.len() > product_expansion_limit {
                return None;
            }
            Some(
                factor_expansion
                    .iter()
                    .map(|&(rnum, ref exponentials)| (rnum, exponentials.to_vec()))
                    .collect(),
            )
        }
    }
}
