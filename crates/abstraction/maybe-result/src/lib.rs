#![feature(try_trait_v2)]

pub use MaybeResult::*;

/// composition of option and result
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MaybeResult<T, E> {
    JustOk(T),
    JustErr(E),
    Nothing,
}

pub enum MaybeResultResidual<E> {
    JustErr(E),
    Nothing,
}

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
            MaybeResultResidual::JustErr(e) => JustErr(e.into()),
            MaybeResultResidual::Nothing => Nothing,
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
