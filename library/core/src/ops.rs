pub use std::ops::*;

pub trait Unveil<T>: Sized {
    type RuntimeConstSymbols;

    type Output;

    fn unveil(
        t: T,
        runtime_const_symbols: &Self::RuntimeConstSymbols,
    ) -> std::ops::ControlFlow<Self, Self::Output>;
}
