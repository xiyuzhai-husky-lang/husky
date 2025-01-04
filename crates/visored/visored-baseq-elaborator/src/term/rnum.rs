use super::*;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqRnumTerm {
    Nat128(u128),
    Int128(i128),
    BigInt(/* TODO */),
    Rat128(i128, u128),
}
