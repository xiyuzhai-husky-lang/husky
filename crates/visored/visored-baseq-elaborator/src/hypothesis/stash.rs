//! The word `stash` is used instead of `cache` because the values are not necessarily up-to-date.
//!
//! Stashes need external sources to provide up-to-update values.
pub mod shared;
pub mod unique;
pub mod upgrade;

pub trait IsVdBsqHypothesisStashScheme {
    type Key<'sess>: std::fmt::Debug + Eq + std::hash::Hash;
    type Value<'sess>: std::fmt::Debug + Eq;
}
