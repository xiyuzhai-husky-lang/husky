mod list;
mod rollback;
mod seq;
#[cfg(test)]
mod tests;

pub use list::*;
pub use rollback::*;
pub use seq::*;

use original_error::OriginalError;
#[cfg(test)]
use tests::*;

pub trait HasStreamState {
    type State;
    fn save_state(&self) -> Self::State;
    fn rollback(&mut self, state: Self::State);
}

pub trait StreamWrapper: std::ops::DerefMut
where
    Self::Target: HasStreamState,
{
}

impl<Wrapper> HasStreamState for Wrapper
where
    Wrapper: StreamWrapper,
    Wrapper::Target: HasStreamState,
{
    type State = <<Wrapper as std::ops::Deref>::Target as HasStreamState>::State;

    fn save_state(&self) -> Self::State {
        self.deref().save_state()
    }

    fn rollback(&mut self, state: Self::State) {
        self.deref_mut().rollback(state)
    }
}

pub trait StreamParser: HasStreamState {
    fn try_parse_optional<P: TryParseOptionalFromStream<Self>>(
        &mut self,
    ) -> Result<Option<P>, <P as TryParseOptionalFromStream<Self>>::Error>;

    #[inline(always)]
    fn try_parse<P: TryParseFromStream<Self>>(
        &mut self,
    ) -> Result<P, <P as TryParseFromStream<Self>>::Error>;

    fn parse_err_as_none<P: TryParseOptionalFromStream<Self>>(&mut self) -> Option<P>;

    fn parse_expected<T: TryParseOptionalFromStream<Self>, E: OriginalError>(
        &mut self,
        f: impl FnOnce(Self::State) -> E,
    ) -> Result<T, E::Error>
    where
        E::Error: From<<T as TryParseOptionalFromStream<Self>>::Error>;

    fn parse_expected_with_context<T: TryParseOptionFromStreamWithContext<Self>, E: OriginalError>(
        &mut self,
        ctx: T::Context,
        f: impl FnOnce(Self::State) -> E,
    ) -> Result<T, E::Error>
    where
        E::Error: From<<T as TryParseOptionFromStreamWithContext<Self>>::Error>;

    /// returns an optional and the rest of the stream,
    ///
    /// guarantees that stream state is not changed if result is Ok(None)
    fn parse_into<P: TryParseOptionalFromStream<Self>>(
        self,
    ) -> Result<(Option<P>, Self), <P as TryParseOptionalFromStream<Self>>::Error>
    where
        Self: Sized;
}

impl<SP> StreamParser for SP
where
    SP: HasStreamState,
{
    #[inline(always)]
    fn try_parse_optional<P: TryParseOptionalFromStream<Self>>(
        &mut self,
    ) -> Result<Option<P>, <P as TryParseOptionalFromStream<Self>>::Error> {
        P::try_parse_from_with_rollback_when_no_error(self)
    }

    #[inline(always)]
    fn try_parse<P: TryParseFromStream<Self>>(
        &mut self,
    ) -> Result<P, <P as TryParseFromStream<Self>>::Error> {
        todo!()
        // P::parse_from_with_rollback_when_no_error(self)
    }

    #[inline(always)]
    fn parse_err_as_none<T: TryParseOptionalFromStream<Self>>(&mut self) -> Option<T> {
        T::parse_option_from_with_rollback_ignoring_error(self)
    }

    #[inline(always)]
    fn parse_expected<T: TryParseOptionalFromStream<Self>, E: OriginalError>(
        &mut self,
        f: impl FnOnce(Self::State) -> E,
    ) -> Result<T, E::Error>
    where
        E::Error: From<<T as TryParseOptionalFromStream<Self>>::Error>,
    {
        let saved_state = self.save_state();
        match T::try_parse_from_with_rollback_when_no_error(self)? {
            Some(output) => Ok(output),
            None => Err(f(saved_state).into()),
        }
    }

    #[inline(always)]
    fn parse_expected_with_context<T: TryParseOptionFromStreamWithContext<Self>, E: OriginalError>(
        &mut self,
        ctx: T::Context,
        f: impl FnOnce(Self::State) -> E,
    ) -> Result<T, E::Error>
    where
        E::Error: From<<T as TryParseOptionFromStreamWithContext<Self>>::Error>,
    {
        let saved_state = self.save_state();
        match T::parse_from_with_rollback_when_no_error(self, ctx)? {
            Some(output) => Ok(output),
            None => Err(f(saved_state).into()),
        }
    }

    #[inline(always)]
    fn parse_into<T: TryParseOptionalFromStream<Self>>(
        mut self,
    ) -> Result<(Option<T>, Self), <T as TryParseOptionalFromStream<Self>>::Error> {
        let optional = T::try_parse_from_with_rollback_when_no_error(&mut self)?;
        Ok((optional, self))
    }
}

pub trait TryParseOptionalFromStream<SP>: Sized
where
    SP: StreamParser + ?Sized,
{
    type Error;

    /// no guarantee on stream state other than Ok(Some(_))
    fn try_parse_optional_from_without_guaranteed_rollback(
        sp: &mut SP,
    ) -> Result<Option<Self>, Self::Error>;
}

pub trait TryParseFromStream<SP>: Sized
where
    SP: StreamParser + ?Sized,
{
    type Error;

    /// no guarantee on stream state other than Ok(Some(_))
    fn try_parse_from_without_guaranteed_rollback(sp: &mut SP) -> Result<Self, Self::Error>;
}

pub trait TryParseOptionFromStreamWithContext<SP>: Sized
where
    SP: StreamParser + ?Sized,
{
    type Error;

    type Context;

    /// no guarantee on stream state other than Ok(Some(_))
    fn try_parse_option_from_without_guaranteed_rollback(
        sp: &mut SP,
        ctx: Self::Context,
    ) -> Result<Option<Self>, Self::Error>;
}
