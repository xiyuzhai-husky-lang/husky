pub mod shared;
pub mod unique;
pub mod update;

pub trait IsVdBsqHypothesisStashScheme {
    type Key<'sess>;
    type Value<'sess>;
}
