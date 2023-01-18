#![feature(try_trait_v2)]

pub use Outcome::*;

use std::convert::Infallible;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Outcome<T, E, A> {
    Success(T),
    Failure(E),
    Abort(A),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Stop<E, A> {
    Failure(E),
    Abort(A),
}

impl<T, E, A> From<Stop<E, A>> for Outcome<T, E, A> {
    fn from(value: Stop<E, A>) -> Self {
        todo!()
    }
}

impl<T, E, A> Outcome<T, E, A> {
    /// Calls op if the result is Ok, otherwise returns the Err value of self.
    pub fn and_then<U>(
        self,
        op: impl FnOnce(T) -> Outcome<U, E, A>,
        ab: impl FnOnce(Stop<E, A>) -> A,
    ) -> Outcome<U, E, A> {
        match self.into_result() {
            Ok(t) => op(t),
            Err(e) => Abort(ab(e)),
        }
    }

    pub fn ok_copy(&self) -> Option<T>
    where
        T: Copy,
    {
        match self {
            Success(t) => Some(*t),
            Failure(_) | Abort(_) => None,
        }
    }

    pub fn ok_copy_err_as_ref<'a>(&'a self) -> Outcome<T, &'a E, &'a A>
    where
        T: Copy,
    {
        match self {
            Success(t) => Success(*t),
            Failure(e) => Failure(e),
            Abort(a) => Abort(a),
        }
    }

    fn into_result(self) -> Result<T, Stop<E, A>> {
        match self {
            Success(t) => Ok(t),
            Failure(f) => Err(Stop::Failure(f)),
            Abort(a) => Err(Stop::Abort(a)),
        }
    }
}

impl<T, E, A> std::ops::Try for Outcome<T, E, A> {
    type Output = T;

    type Residual = Outcome<Infallible, E, A>;

    fn from_output(output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Success(t) => std::ops::ControlFlow::Continue(t),
            Failure(_) => todo!(),
            Abort(_) => todo!(),
        }
    }
}

impl<T, E, A> std::ops::FromResidual<Outcome<Infallible, E, A>> for Outcome<T, E, A> {
    fn from_residual(residual: Outcome<Infallible, E, A>) -> Self {
        todo!()
    }
}

impl<T, E1, E2, A> std::ops::FromResidual<Result<Infallible, E1>> for Outcome<T, E2, A>
where
    E2: From<E1>,
{
    fn from_residual(residual: Result<Infallible, E1>) -> Self {
        todo!()
    }
}
