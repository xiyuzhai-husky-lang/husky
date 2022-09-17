#![feature(try_trait_v2)]

pub trait Monad: std::ops::Try {}

pub trait MonadTransFrom<T>: std::ops::FromResidual<T> {}
