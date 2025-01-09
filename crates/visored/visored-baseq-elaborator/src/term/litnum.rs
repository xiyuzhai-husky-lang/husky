pub mod bigint;
pub mod bigrat;
pub mod frac128;

use self::{bigint::VdBsqBigInt, frac128::VdBsqFrac128};
use super::*;
use std::num::NonZeroU128;
use visored_opr::precedence::{VdPrecedence, VdPrecedenceRange};

#[enum_class::from_variants]
#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqLitnumTerm<'sess> {
    Int128(i128),
    BigInt(VdBsqBigInt<'sess>),
    Frac128(VdBsqFrac128),
}

impl<'sess> std::fmt::Debug for VdBsqLitnumTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("VdBsqLitnumTerm(`")?;
        self.show_fmt(VdPrecedenceRange::Any, f)?;
        f.write_str("`)")
    }
}

impl<'sess> VdBsqLitnumTerm<'sess> {
    pub fn neg(self, db: &FloaterDb) -> Self {
        match self {
            VdBsqLitnumTerm::Int128(i) => VdBsqLitnumTerm::Int128(-i),
            VdBsqLitnumTerm::BigInt(i) => todo!(),
            VdBsqLitnumTerm::Frac128(f) => VdBsqLitnumTerm::Frac128(-f),
        }
    }

    pub fn add_assign(&mut self, rhs: Self, db: &FloaterDb) {
        if self.is_zero() {
            *self = rhs;
            return;
        }
        match self {
            VdBsqLitnumTerm::Int128(slf) => match rhs {
                VdBsqLitnumTerm::Int128(rhs) => match slf.checked_add(rhs) {
                    Some(sum) => *self = VdBsqLitnumTerm::Int128(sum),
                    None => todo!(),
                },
                VdBsqLitnumTerm::BigInt(i) => {
                    use husky_print_utils::p;
                    p!(self, rhs);
                    todo!()
                }
                VdBsqLitnumTerm::Frac128(_) => todo!(),
            },
            VdBsqLitnumTerm::BigInt(i) => todo!(),
            VdBsqLitnumTerm::Frac128(_) => todo!(),
        }
    }

    pub fn sub_assign(&mut self, rhs: Self, db: &'sess FloaterDb) {
        match self {
            VdBsqLitnumTerm::Int128(slf) => match rhs {
                VdBsqLitnumTerm::Int128(rhs) => match slf.checked_sub(rhs) {
                    Some(sum) => *self = VdBsqLitnumTerm::Int128(sum),
                    None => todo!(),
                },
                VdBsqLitnumTerm::BigInt(i) => todo!(),
                VdBsqLitnumTerm::Frac128(_) => todo!(),
            },
            VdBsqLitnumTerm::BigInt(i) => match rhs {
                VdBsqLitnumTerm::Int128(_) => todo!(),
                VdBsqLitnumTerm::BigInt(i1) => *self = i.sub(i1, db),
                VdBsqLitnumTerm::Frac128(_) => todo!(),
            },
            VdBsqLitnumTerm::Frac128(_) => todo!(),
        }
    }

    pub fn mul(self, rhs: Self, db: &'sess FloaterDb) -> Self {
        match rhs {
            VdBsqLitnumTerm::Int128(rhs) => self.mul128(rhs, db),
            VdBsqLitnumTerm::BigInt(i) => todo!(),
            VdBsqLitnumTerm::Frac128(_) => todo!(),
        }
    }

    pub fn mul128(self, rhs: i128, db: &'sess FloaterDb) -> Self {
        match self {
            VdBsqLitnumTerm::Int128(i) => match i.checked_mul(rhs) {
                Some(product) => VdBsqLitnumTerm::Int128(product),
                None => todo!(),
            },
            VdBsqLitnumTerm::BigInt(i) => todo!(),
            VdBsqLitnumTerm::Frac128(_) => todo!(),
        }
    }

    pub fn mul_assign(&mut self, rhs: Self, db: &'sess FloaterDb) {
        match *self {
            VdBsqLitnumTerm::ZERO => (),
            VdBsqLitnumTerm::ONE => *self = rhs,
            VdBsqLitnumTerm::Int128(slf) => match rhs {
                VdBsqLitnumTerm::Int128(rhs) => match slf.checked_mul(rhs) {
                    Some(product) => *self = VdBsqLitnumTerm::Int128(product),
                    None => todo!(),
                },
                VdBsqLitnumTerm::BigInt(i) => todo!(),
                VdBsqLitnumTerm::Frac128(f) => todo!(),
            },
            VdBsqLitnumTerm::BigInt(i) => todo!(),
            VdBsqLitnumTerm::Frac128(f) => *self = f.mul_litn(rhs, db),
        }
    }

    pub fn div_assign(&mut self, rhs: Self, db: &FloaterDb) {
        match *self {
            VdBsqLitnumTerm::Int128(slf) => match rhs {
                VdBsqLitnumTerm::Int128(rhs) => *self = VdBsqFrac128::new128(slf, rhs),
                VdBsqLitnumTerm::BigInt(i) => todo!(),
                VdBsqLitnumTerm::Frac128(_) => todo!(),
            },
            VdBsqLitnumTerm::BigInt(i) => todo!(),
            VdBsqLitnumTerm::Frac128(_) => todo!(),
        }
    }

    pub fn pow128(self, exponent: i128, db: &FloaterDb) -> Self {
        match self {
            VdBsqLitnumTerm::Int128(i) => {
                if exponent > 0 {
                    let exponent: u32 = match exponent.try_into() {
                        Ok(exponent) => exponent,
                        Err(_) => todo!(),
                    };
                    match i.checked_pow(exponent) {
                        Some(pow) => VdBsqLitnumTerm::Int128(pow),
                        None => todo!(),
                    }
                } else {
                    use husky_print_utils::p;
                    p!(exponent);
                    todo!()
                }
            }
            VdBsqLitnumTerm::BigInt(i) => todo!(),
            VdBsqLitnumTerm::Frac128(_) => todo!(),
        }
    }
}

impl<'sess> VdBsqLitnumTerm<'sess> {
    pub const ZERO: Self = Self::Int128(0);
    pub const ONE: Self = Self::Int128(1);
    pub const NEG_ONE: Self = Self::Int128(-1);
}

impl<'sess> VdBsqLitnumTerm<'sess> {
    pub fn is_zero(self) -> bool {
        self.eqs_i128(0)
    }

    pub fn is_one(self) -> bool {
        self.eqs_i128(1)
    }

    pub fn eqs_i128(self, i0: i128) -> bool {
        match self {
            VdBsqLitnumTerm::Int128(i) => i == i0,
            _ => false,
        }
    }
}

impl<'sess> VdBsqLitnumTerm<'sess> {
    pub fn show_fmt(
        self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let outer_precedence = self.outer_precedence();
        if precedence_range.contains(outer_precedence) {
            self.show_fmt_inner(f)
        } else {
            f.write_str("(")?;
            self.show_fmt_inner(f)?;
            f.write_str(")")
        }
    }

    pub fn outer_precedence(self) -> VdPrecedence {
        match self {
            VdBsqLitnumTerm::Int128(i) => {
                if i >= 0 {
                    VdPrecedence::ATOM
                } else {
                    VdPrecedence::ADD_SUB
                }
            }
            VdBsqLitnumTerm::BigInt(i) => {
                if i.is_nonnegative() {
                    VdPrecedence::ATOM
                } else {
                    VdPrecedence::ADD_SUB
                }
            }
            VdBsqLitnumTerm::Frac128(_) => todo!(),
        }
    }

    fn show_fmt_inner(self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VdBsqLitnumTerm::Int128(i) => write!(f, "{}", i),
            VdBsqLitnumTerm::BigInt(i) => i.show_fmt(f),
            VdBsqLitnumTerm::Frac128(_) => todo!(),
        }
    }
}

impl<'sess> VdBsqLitnumTerm<'sess> {
    pub fn compare_with_zero(self, kind: VdBsqNumRelationshipPropTermKind) -> bool {
        match self {
            VdBsqLitnumTerm::Int128(i) => match kind {
                VdBsqNumRelationshipPropTermKind::Eq => i == 0,
                VdBsqNumRelationshipPropTermKind::Ne => i != 0,
                VdBsqNumRelationshipPropTermKind::Lt => i < 0,
                VdBsqNumRelationshipPropTermKind::Gt => i > 0,
                VdBsqNumRelationshipPropTermKind::Le => i <= 0,
                VdBsqNumRelationshipPropTermKind::Ge => i >= 0,
            },
            VdBsqLitnumTerm::BigInt(i) => todo!(),
            VdBsqLitnumTerm::Frac128(_) => todo!(),
        }
    }
}
