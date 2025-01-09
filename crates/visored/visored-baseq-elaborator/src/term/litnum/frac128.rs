use super::*;
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VdBsqFrac128 {
    numerator: i128,
    denominator: i128,
}

impl VdBsqFrac128 {
    pub fn new128<'sess>(
        raw_numerator: i128,
        raw_denominator: i128,
    ) -> Option<VdBsqLitnumTerm<'sess>> {
        if raw_denominator == 0 {
            return None;
        }
        let (numerator, denominator) = reduce(raw_numerator, raw_denominator);
        debug_assert!(denominator > 0);
        debug_assert_eq!(
            BigInt::from(denominator) * BigInt::from(raw_numerator),
            BigInt::from(numerator) * BigInt::from(raw_denominator)
        );
        if denominator == 1 {
            Some(numerator.into())
        } else {
            Some(
                Self {
                    numerator,
                    denominator,
                }
                .into(),
            )
        }
    }

    pub fn new_i128_inverse(i: i128) -> Option<VdBsqFrac128> {
        debug_assert!(i != 0);
        if i > 0 {
            Some(Self {
                numerator: 1,
                denominator: i,
            })
        } else {
            Some(Self {
                numerator: -1,
                denominator: i.checked_neg()?,
            })
        }
    }
}

fn reduce(raw_numerator: i128, raw_denominator: i128) -> (i128, i128) {
    if raw_numerator == i128::MIN || raw_denominator == i128::MIN {
        todo!("needs careful handling")
    } else {
        use num_integer::Integer;

        let gcd = raw_numerator.abs().gcd(&raw_denominator.abs());
        let (numerator, denominator) = (raw_numerator / gcd, raw_denominator / gcd);
        if denominator < 0 {
            (-numerator, -denominator)
        } else {
            (numerator, denominator)
        }
    }
}

impl VdBsqFrac128 {
    pub fn numerator(self) -> i128 {
        self.numerator
    }

    pub fn denominator(self) -> i128 {
        self.denominator
    }
}

impl std::ops::Neg for VdBsqFrac128 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl<'sess> VdBsqFrac128 {
    pub fn mul_litn(
        self,
        rhs: VdBsqLitnumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> VdBsqLitnumTerm<'sess> {
        match rhs {
            VdBsqLitnumTerm::Int128(i) => self.mul_i128(i, db),
            VdBsqLitnumTerm::BigInt(vd_bsq_litnum_term_big_int) => todo!(),
            VdBsqLitnumTerm::Frac128(vd_bsq_frac128) => todo!(),
        }
    }

    pub fn mul_i128(self, rhs: i128, db: &'sess FloaterDb) -> VdBsqLitnumTerm<'sess> {
        let Self {
            numerator,
            denominator,
        } = self;
        match numerator.checked_mul(rhs) {
            Some(raw_numerator) => Self::new128(raw_numerator, denominator).unwrap(),
            None => todo!(),
        }
    }
}
