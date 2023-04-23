#![feature(try_trait_v2)]

pub use MaybeResult::*;

use original_error::OriginalError;
use std::convert::Infallible;

/// composition of option and result
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MaybeResult<T, E> {
    JustOk(T),
    JustErr(E),
    Nothing,
}

type MaybeResultResidual<E> = MaybeResult<Infallible, E>;

impl<T, E> std::ops::Try for MaybeResult<T, E> {
    type Output = T;

    type Residual = MaybeResultResidual<E>;

    fn from_output(t: Self::Output) -> Self {
        JustOk(t)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            JustOk(t) => std::ops::ControlFlow::Continue(t),
            JustErr(e) => std::ops::ControlFlow::Break(MaybeResultResidual::JustErr(e)),
            Nothing => std::ops::ControlFlow::Break(MaybeResultResidual::Nothing),
        }
    }
}

impl<T, E1, E2> std::ops::FromResidual<MaybeResultResidual<E1>> for MaybeResult<T, E2>
where
    E2: From<E1>,
{
    fn from_residual(residual: MaybeResultResidual<E1>) -> Self {
        match residual {
            JustOk(_) => unreachable!(),
            JustErr(e) => JustErr(e.into()),
            Nothing => Nothing,
        }
    }
}

impl<T, E1, E2> std::ops::FromResidual<Result<Infallible, E1>> for MaybeResult<T, E2>
where
    E2: From<E1>,
{
    fn from_residual(residual: Result<Infallible, E1>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(_) => todo!(),
        }
    }
}

#[test]
fn maybe_result_works() {
    assert_eq!(
        || -> MaybeResult<i32, ()> {
            JustOk(0)?;
            JustOk(1)
        }(),
        JustOk(1)
    );
    assert_eq!(
        || -> MaybeResult<i32, ()> {
            JustErr(())?;
            JustOk(1)
        }(),
        JustErr(())
    );
    assert_eq!(
        || -> MaybeResult<i32, ()> {
            Nothing?;
            JustOk(1)
        }(),
        Nothing
    );
}

impl<T, E> MaybeResult<T, E> {
    /// convert into `Result<Option<T>, E>`
    ///
    /// ```
    /// use maybe_result::*;
    /// let a: MaybeResult<i32, ()> = JustOk(1);
    /// assert_eq!(a.into_result_option(), Ok(Some(1)));
    /// let b: MaybeResult<i32, ()> = JustErr(());
    /// assert_eq!(b.into_result_option(), Err(()));
    /// let c: MaybeResult<i32, ()> = Nothing;
    /// assert_eq!(c.into_result_option(), Ok(None));
    /// ```
    pub fn into_result_option(self) -> Result<Option<T>, E> {
        match self {
            JustOk(t) => Ok(Some(t)),
            JustErr(e) => Err(e),
            Nothing => Ok(None),
        }
    }

    pub fn into_result_or<OE>(self, nothing_e: OE) -> Result<T, <OE as OriginalError>::Error>
    where
        OE: OriginalError,
        E: Into<<OE as OriginalError>::Error>,
    {
        match self {
            JustOk(t) => Ok(t),
            JustErr(e) => Err(e.into()),
            Nothing => Err(nothing_e.into()),
        }
    }

    pub fn just_ok_as_ref(&self) -> MaybeResult<&T, E>
    where
        E: Copy,
    {
        match self {
            JustOk(t) => JustOk(t),
            JustErr(e) => JustErr(*e),
            Nothing => Nothing,
        }
    }
}
