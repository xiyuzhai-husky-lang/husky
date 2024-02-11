pub mod helpers;
pub mod macros;
pub mod vec;
pub mod from_str_pos {
    pub use from_str_pos::*;
}

/// A trait for newtyped integers, that can be used as index types in vectors and sets.
pub trait Idx: Copy + Eq + std::hash::Hash + Ord + std::fmt::Debug + Default {
    /// Convert from `T` to `usize`
    fn into_usize(self) -> usize;
    /// Convert from `usize` to `T`
    fn from_usize(_: usize) -> Self;
    /// Generate a fresh variable from a `&mut ID` counter.
    #[must_use]
    fn fresh(&mut self) -> Self {
        let n = *self;
        *self = Self::from_usize(self.into_usize() + 1);
        n
    }
}

impl Idx for usize {
    fn into_usize(self) -> usize {
        self
    }
    fn from_usize(n: usize) -> Self {
        n
    }
}
impl Idx for u32 {
    fn into_usize(self) -> usize {
        self as _
    }
    fn from_usize(n: usize) -> Self {
        n as _
    }
}
