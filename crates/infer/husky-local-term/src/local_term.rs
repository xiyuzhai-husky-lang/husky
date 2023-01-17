mod curry;

pub use curry::*;

use crate::*;

pub enum LocalTerm {
    Term(Term),
    ImplicitLifetime(u8),
    // Xin Jiang: add other variants
}
