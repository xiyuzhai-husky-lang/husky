#![feature(try_trait_v2)]

pub trait Monad: std::ops::Try {}

pub trait MonadT<T: Monad>: std::ops::FromResidual<T::Residual> {}
