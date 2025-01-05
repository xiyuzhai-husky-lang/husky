//! rust version of Haskell Alternative monad
//!
//! ```
//! use alt_option::*;
//! struct Shop {}
//!
//! enum Food {}
//!
//! impl Shop {
//!     fn food(&self) -> AltOption<Food> {
//!         todo!("...")
//!     }
//! }
//! ```
#![feature(try_trait_v2)]
use std::convert::Infallible;

pub use AltOption::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AltOption<T> {
    AltSome(T),
    AltNone,
}

pub struct AltOptionR<T>(T);

impl<T> std::ops::Try for AltOption<T> {
    type Output = ();

    type Residual = AltOptionR<T>;

    fn from_output(_output: Self::Output) -> Self {
        AltNone
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            AltSome(t) => std::ops::ControlFlow::Break(AltOptionR(t)),
            AltNone => std::ops::ControlFlow::Continue(()),
        }
    }
}

impl<T> std::ops::FromResidual<AltOptionR<T>> for AltOption<T> {
    fn from_residual(residual: AltOptionR<T>) -> Self {
        AltOption::AltSome(residual.0)
    }
}

impl<T> std::ops::FromResidual<Option<Infallible>> for AltOption<T> {
    fn from_residual(_residual: Option<Infallible>) -> Self {
        AltNone
    }
}

impl<T> Into<Option<T>> for AltOption<T> {
    fn into(self) -> Option<T> {
        match self {
            AltSome(t) => Some(t),
            AltNone => None,
        }
    }
}

impl<T> From<Option<T>> for AltOption<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(t) => AltSome(t),
            None => AltNone,
        }
    }
}

#[macro_export]
macro_rules! try_alt {
    ($($e:expr),+) => {
        $(
            if let AltSome(value) = $e? {
                return Ok(AltSome(value));
            }
        )+
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_some(x: i32) -> Result<AltOption<i32>, ()> {
        Ok(AltSome(x))
    }

    fn get_none() -> Result<AltOption<i32>, ()> {
        Ok(AltNone)
    }

    fn try_single() -> Result<AltOption<i32>, ()> {
        try_alt!(get_none());
        Ok(AltNone)
    }

    fn try_single_some() -> Result<AltOption<i32>, ()> {
        try_alt!(get_some(42));
        Ok(AltNone)
    }

    fn try_multiple() -> Result<AltOption<i32>, ()> {
        try_alt!(get_none(), get_some(42), get_some(24));
        Ok(AltNone)
    }

    fn try_all_none() -> Result<AltOption<i32>, ()> {
        try_alt!(get_none(), get_none(), get_none());
        Ok(AltNone)
    }

    #[test]
    fn test_try_alt() {
        assert_eq!(try_single(), Ok(AltNone));
        assert_eq!(try_single_some(), Ok(AltSome(42)));
        assert_eq!(try_multiple(), Ok(AltSome(42)));
        assert_eq!(try_all_none(), Ok(AltNone));
    }
}
