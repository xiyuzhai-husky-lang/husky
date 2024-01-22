use serde::{Deserialize, Serialize};

pub trait IsPedestal:
    std::fmt::Debug
    + Default
    + PartialEq
    + Eq
    + Clone
    + Copy
    + Send
    + Sync
    + Serialize
    + std::hash::Hash
    + for<'a> Deserialize<'a>
    + 'static
{
}

impl IsPedestal for () {}
