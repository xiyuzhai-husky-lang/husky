#![feature(try_trait_v2_residual)]
#![feature(try_trait_v2)]

pub use AltMaybeResult::*;

use alt_option::*;
use original_error::OriginalError;
use std::convert::Infallible;

/// composition of option and result
#[must_use]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AltMaybeResult<T, E> {
    AltJustOk(T),
    AltJustErr(E),
    AltNothing,
}

impl<T, E> Default for AltMaybeResult<T, E> {
    fn default() -> Self {
        AltNothing
    }
}

impl<T, E> AltMaybeResult<T, E> {
    #[track_caller]
    pub fn expect(self, message: &str) -> T {
        match self {
            AltJustOk(t) => t,
            AltJustErr(_) => panic!("{message}"),
            AltNothing => panic!("{message}"),
        }
    }

    pub fn map_err_or_none<E2>(self, f: impl FnOnce(Option<E>) -> E2) -> Result<T, E2> {
        match self {
            AltJustOk(t) => Ok(t),
            AltJustErr(e) => Err(f(Some(e))),
            AltNothing => Err(f(None)),
        }
    }
}

impl<T, E> From<Result<T, E>> for AltMaybeResult<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(t) => AltJustOk(t),
            Err(e) => AltJustErr(e),
        }
    }
}

pub enum AltMaybeResultResidual<T, E> {
    AltJustOk(T),
    AltJustErr(E),
}

impl<T, E> std::ops::Try for AltMaybeResult<T, E> {
    type Output = ();

    type Residual = AltMaybeResultResidual<T, E>;

    fn from_output(_: Self::Output) -> Self {
        AltNothing
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            AltJustOk(t) => std::ops::ControlFlow::Break(AltMaybeResultResidual::AltJustOk(t)),
            AltJustErr(e) => std::ops::ControlFlow::Break(AltMaybeResultResidual::AltJustErr(e)),
            AltNothing => std::ops::ControlFlow::Continue(()),
        }
    }
}

impl<T, E1, E2> std::ops::FromResidual<AltMaybeResultResidual<T, E1>> for AltMaybeResult<T, E2>
where
    E2: From<E1>,
{
    fn from_residual(residual: AltMaybeResultResidual<T, E1>) -> Self {
        match residual {
            AltMaybeResultResidual::AltJustOk(t) => AltJustOk(t),
            AltMaybeResultResidual::AltJustErr(e) => AltJustErr(e.into()),
        }
    }
}

impl<T, E> std::ops::FromResidual<Option<Infallible>> for AltMaybeResult<T, E> {
    fn from_residual(residual: Option<Infallible>) -> Self {
        AltNothing
    }
}

impl<T, E1, E2> std::ops::FromResidual<Result<Infallible, E1>> for AltMaybeResult<T, E2>
where
    E2: From<E1>,
{
    fn from_residual(residual: Result<Infallible, E1>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => AltJustErr(e.into()),
        }
    }
}

impl<T, E> std::ops::FromResidual<AltOptionResidual<T>> for AltMaybeResult<T, E> {
    fn from_residual(residual: AltOptionResidual<T>) -> Self {
        AltJustOk(residual.into_inner())
    }
}

#[test]
fn alt_maybe_result_works() {
    assert_eq!(
        || -> AltMaybeResult<i32, ()> {
            AltJustOk(0)?;
            AltJustOk(1)
        }(),
        AltJustOk(0)
    );
    assert_eq!(
        || -> AltMaybeResult<i32, ()> {
            AltJustErr(())?;
            AltJustOk(1)
        }(),
        AltJustErr(())
    );
    assert_eq!(
        || -> AltMaybeResult<i32, ()> {
            AltNothing?;
            AltJustOk(1)
        }(),
        AltJustOk(1)
    );
}

impl<T, E> From<Result<T, AltOption<E>>> for AltMaybeResult<T, E> {
    fn from(value: Result<T, AltOption<E>>) -> Self {
        match value {
            Ok(t) => AltJustOk(t),
            Err(AltOption::AltSome(e)) => AltJustErr(e),
            Err(AltOption::AltNone) => AltNothing,
        }
    }
}

impl<T, E> AltMaybeResult<T, E> {
    pub fn ok(self) -> AltOption<T> {
        match self {
            AltJustOk(t) => AltSome(t),
            AltJustErr(_) | AltNothing => AltNone,
        }
    }

    /// convert into `Result<Option<T>, E>`
    ///
    /// ```
    /// use alt_maybe_result::*;
    /// use alt_option::*;
    ///
    /// let a: AltMaybeResult<i32, ()> = AltJustOk(1);
    /// assert_eq!(a.into_result_alt_option(), Ok(AltSome(1)));
    /// let b: AltMaybeResult<i32, ()> = AltJustErr(());
    /// assert_eq!(b.into_result_alt_option(), Err(()));
    /// let c: AltMaybeResult<i32, ()> = AltNothing;
    /// assert_eq!(c.into_result_alt_option(), Ok(AltNone));
    /// ```
    pub fn into_result_alt_option(self) -> Result<AltOption<T>, E> {
        match self {
            AltJustOk(t) => Ok(AltSome(t)),
            AltJustErr(e) => Err(e),
            AltNothing => Ok(AltNone),
        }
    }

    pub fn into_alt_option_result(self) -> AltOption<Result<T, E>> {
        match self {
            AltJustOk(t) => AltSome(Ok(t)),
            AltJustErr(e) => AltSome(Err(e)),
            AltNothing => AltNone,
        }
    }

    pub fn into_result_alt_option_err(self) -> Result<T, AltOption<E>> {
        match self {
            AltJustOk(t) => Ok(t),
            AltJustErr(e) => Err(AltSome(e)),
            AltNothing => Err(AltNone),
        }
    }

    pub fn into_result_or<OE>(self, nothing_e: OE) -> Result<T, <OE as OriginalError>::Error>
    where
        OE: OriginalError,
        E: Into<<OE as OriginalError>::Error>,
    {
        match self {
            AltJustOk(t) => Ok(t),
            AltJustErr(e) => Err(e.into()),
            AltNothing => Err(nothing_e.into()),
        }
    }

    pub fn just_ok_as_ref(&self) -> AltMaybeResult<&T, E>
    where
        E: Copy,
    {
        match self {
            AltJustOk(t) => AltJustOk(t),
            AltJustErr(e) => AltJustErr(*e),
            AltNothing => AltNothing,
        }
    }

    pub fn just_ok_as_ref2<S: ?Sized>(&self) -> AltMaybeResult<&S, E>
    where
        E: Copy,
        T: AsRef<S>,
    {
        match self {
            AltJustOk(t) => AltJustOk(t.as_ref()),
            AltJustErr(e) => AltJustErr(*e),
            AltNothing => AltNothing,
        }
    }

    pub fn map<S>(self, f: impl FnOnce(T) -> S) -> AltMaybeResult<S, E> {
        match self {
            AltJustOk(t) => AltJustOk(f(t)),
            AltJustErr(e) => AltJustErr(e),
            AltNothing => AltNothing,
        }
    }
}

impl<T, E> std::ops::Residual<()> for AltMaybeResultResidual<T, E> {
    type TryType = AltMaybeResult<T, E>;
}
