pub use std::ops::*;

pub trait Unveil<T>: Sized {
    type Output;

    fn unveil(t: T) -> std::ops::ControlFlow<Self, Self::Output>;
}
