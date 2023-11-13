pub mod full_map;

pub use enum_index_macros::*;

pub trait IsEnumIndex: Copy {
    const N: usize;

    fn from_index(index: usize) -> Self;

    fn index(self) -> usize;
}
