mod list;
mod rollback;
mod seq;
#[cfg(test)]
mod tests;

pub use list::*;
pub use rollback::*;
pub use seq::*;

#[cfg(test)]
use tests::*;

pub trait OriginalError: Sized {
    type Error: From<Self>;
}

impl OriginalError for () {
    type Error = Self;
}

pub trait HasParseState {
    type State;
    fn save_state(&self) -> Self::State;
    fn rollback(&mut self, state: Self::State);
}

pub trait StreamWrapper: std::ops::DerefMut
where
    Self::Target: HasParseState,
{
}

impl<Wrapper> HasParseState for Wrapper
where
    Wrapper: StreamWrapper,
    Wrapper::Target: HasParseState,
{
    type State = <<Wrapper as std::ops::Deref>::Target as HasParseState>::State;

    fn save_state(&self) -> Self::State {
        self.deref().save_state()
    }

    fn rollback(&mut self, state: Self::State) {
        self.deref_mut().rollback(state)
    }
}

pub trait ParseContext: HasParseState {
    fn parse<P: ParseFrom<Self>>(&mut self) -> Result<Option<P>, <P as ParseFrom<Self>>::Error>;
    fn try_parse<P: ParseFrom<Self>>(&mut self) -> Option<P>;

    fn parse_expected<P: ParseFrom<Self>, E: OriginalError>(
        &mut self,
        f: impl FnOnce(Self::State) -> E,
    ) -> Result<P, E::Error>
    where
        E::Error: From<<P as ParseFrom<Self>>::Error>;

    /// returns an optional and the rest of the stream,
    ///
    /// guarantees that stream state is not changed if result is Ok(None)
    fn parse_into<P: ParseFrom<Self>>(
        self,
    ) -> Result<(Option<P>, Self), <P as ParseFrom<Self>>::Error>
    where
        Self: Sized;
}

impl<Context> ParseContext for Context
where
    Context: HasParseState,
{
    fn parse<P: ParseFrom<Self>>(&mut self) -> Result<Option<P>, <P as ParseFrom<Self>>::Error> {
        P::parse_from_with_rollback_when_no_error(self)
    }

    /// deprecated
    fn try_parse<P: ParseFrom<Self>>(&mut self) -> Option<P> {
        P::try_parse_from_with_rollback(self)
    }

    fn parse_expected<P: ParseFrom<Self>, E: OriginalError>(
        &mut self,
        f: impl FnOnce(Self::State) -> E,
    ) -> Result<P, E::Error>
    where
        E::Error: From<<P as ParseFrom<Self>>::Error>,
    {
        let saved_state = self.save_state();
        match P::parse_from_with_rollback_when_no_error(self)? {
            Some(output) => Ok(output),
            None => Err(f(saved_state).into()),
        }
    }

    fn parse_into<P: ParseFrom<Self>>(
        mut self,
    ) -> Result<(Option<P>, Self), <P as ParseFrom<Self>>::Error> {
        let optional = P::parse_from_with_rollback_when_no_error(&mut self)?;
        Ok((optional, self))
    }
}

pub trait ParseFrom<Context>: Sized
where
    Context: ParseContext + ?Sized,
{
    type Error;

    /// no guarantee on stream state other than Ok(Some(_))
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, Self::Error>;
}
