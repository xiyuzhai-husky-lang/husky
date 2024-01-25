#![feature(generic_const_exprs)]
pub mod bitset;
pub mod full_map;

pub use enum_index_macros::*;

pub trait IsEnumIndex: Copy + Eq + 'static {
    const N: usize;

    fn from_index(index: usize) -> Self;

    fn index(self) -> usize;

    fn all() -> impl Iterator<Item = Self> {
        (0..Self::N)
            .into_iter()
            .map(|index| Self::from_index(index))
    }
}
