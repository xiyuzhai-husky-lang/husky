use super::*;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqRnumTerm {
    Nat128(u128),
    Int128(i128),
    BigInt(/* TODO */),
    Rat128(i128, u128),
}

impl std::ops::AddAssign for VdBsqRnumTerm {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            VdBsqRnumTerm::Nat128(slf) => match rhs {
                VdBsqRnumTerm::Nat128(rhs) => match slf.checked_add(rhs) {
                    Some(sum) => *self = Self::Nat128(sum),
                    None => todo!(),
                },
                VdBsqRnumTerm::Int128(_) => todo!(),
                VdBsqRnumTerm::BigInt() => todo!(),
                VdBsqRnumTerm::Rat128(_, _) => todo!(),
            },
            VdBsqRnumTerm::Int128(slf) => match rhs {
                VdBsqRnumTerm::Nat128(_) => todo!(),
                VdBsqRnumTerm::Int128(rhs) => match slf.checked_add(rhs) {
                    Some(sum) => *self = Self::Int128(sum),
                    None => todo!(),
                },
                VdBsqRnumTerm::BigInt() => todo!(),
                VdBsqRnumTerm::Rat128(_, _) => todo!(),
            },
            VdBsqRnumTerm::BigInt() => todo!(),
            VdBsqRnumTerm::Rat128(_, _) => todo!(),
        }
    }
}

impl VdBsqRnumTerm {
    pub const ZERO: Self = Self::Int128(0);
    pub const ONE: Self = Self::Int128(1);
}
