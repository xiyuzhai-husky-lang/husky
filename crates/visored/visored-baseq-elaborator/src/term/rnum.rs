use std::num::NonZeroU128;

use super::*;

#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqRnumTerm {
    Int128(i128),
    BigInt(/* TODO */),
    Rat128(i128, u128),
}

impl std::fmt::Debug for VdBsqRnumTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("`")?;
        self.show_fmt(f)?;
        f.write_str("`")
    }
}

impl std::ops::AddAssign for VdBsqRnumTerm {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            VdBsqRnumTerm::Int128(slf) => match rhs {
                VdBsqRnumTerm::Int128(rhs) => match slf.checked_add(rhs) {
                    Some(sum) => *self = VdBsqRnumTerm::Int128(sum),
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

impl VdBsqRnumTerm {
    pub fn is_zero(self) -> bool {
        self.eqs_i128(0)
    }

    pub fn eqs_i128(self, i0: i128) -> bool {
        match self {
            VdBsqRnumTerm::Int128(i) => i == i0,
            VdBsqRnumTerm::BigInt() => todo!(),
            VdBsqRnumTerm::Rat128(_, _) => todo!(),
        }
    }
}

impl<'sess> VdBsqRnumTerm {
    pub fn show_fmt(self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VdBsqRnumTerm::Int128(i) => write!(f, "{}", i),
            VdBsqRnumTerm::BigInt() => todo!(),
            VdBsqRnumTerm::Rat128(_, _) => todo!(),
        }
    }
}
