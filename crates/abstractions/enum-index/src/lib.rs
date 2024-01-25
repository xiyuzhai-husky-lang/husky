#![feature(generic_const_exprs)]
pub mod bitset;
pub mod full_map;

pub use enum_index_macros::*;

pub trait IsEnumIndex: Copy + Eq {
    const N: usize;

    fn from_index(index: usize) -> Self;

    fn index(self) -> usize;
}
