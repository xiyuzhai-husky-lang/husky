pub mod shared;
pub mod unique;
pub mod update;

pub trait IsVdBsqHypothesisCacheScheme {
    type Key<'sess>;
    type Value<'sess>;
}
