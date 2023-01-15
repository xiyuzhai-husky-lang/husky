mod absent;
mod rollback;
mod separated_list;
mod seq;
#[cfg(test)]
mod tests;

pub use absent::*;
use outcome::*;
pub use rollback::*;
pub use separated_list::*;
pub use seq::*;

#[cfg(test)]
use tests::*;

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

pub trait HasParseError {
    type Error;
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

pub trait ParseContext: HasParseError + HasParseState {
    fn parse<P: ParseFrom<Self>>(&mut self) -> Result<Option<P>, Self::Error>;
    fn try_parse<P: ParseFrom<Self>>(&mut self) -> Option<P>;
    fn parse_expected<P: ParseFrom<Self>>(&mut self) -> Result<P, Self::Error>
    where
        Self::Error: FromAbsent<P, Self>;

    fn parse_expected2<P: ParseFrom<Self>, Error>(
        &mut self,
        f: impl FnOnce(Self::State) -> Error,
    ) -> Result<P, Error>
    where
        Error: From<Self::Error>;

    fn parse_expected_outcome<P: ParseFrom<Self>, Error, Abortion>(
        &mut self,
        f: impl FnOnce(Self::State) -> Stop<Error, Abortion>,
    ) -> Outcome<P, Error, Abortion>
    where
        Error: From<Self::Error>;

    /// returns an optional and the rest of the stream,
    ///
    /// guarantees that stream state is not changed if result is Ok(None)
    fn parse_into<P: ParseFrom<Self>>(self) -> Result<(Option<P>, Self), Self::Error>
    where
        Self: Sized;
}

impl<Context> ParseContext for Context
where
    Context: HasParseError + HasParseState,
{
    fn parse<P: ParseFrom<Self>>(&mut self) -> Result<Option<P>, Self::Error> {
        P::parse_from_with_rollback_when_no_error(self)
    }

    fn try_parse<P: ParseFrom<Self>>(&mut self) -> Option<P> {
        P::try_parse_from_with_rollback(self)
    }

    fn parse_expected<P: ParseFrom<Self>>(&mut self) -> Result<P, Self::Error>
    where
        Self::Error: FromAbsent<P, Self>,
    {
        let saved_state = self.save_state();
        match P::parse_from_with_rollback_when_no_error(self)? {
            Some(output) => Ok(output),
            None => Err(<Self::Error as FromAbsent<P, Context>>::new_absent_error(
                saved_state,
            )),
        }
    }

    fn parse_expected2<P: ParseFrom<Self>, Error>(
        &mut self,
        f: impl FnOnce(Self::State) -> Error,
    ) -> Result<P, Error>
    where
        Error: From<Self::Error>,
    {
        let saved_state = self.save_state();
        match P::parse_from_with_rollback_when_no_error(self)? {
            Some(output) => Ok(output),
            None => Err(f(saved_state)),
        }
    }

    fn parse_expected_outcome<P: ParseFrom<Self>, Error, Abortion>(
        &mut self,
        f: impl FnOnce(Self::State) -> Stop<Error, Abortion>,
    ) -> Outcome<P, Error, Abortion>
    where
        Error: From<Self::Error>,
    {
        let saved_state = self.save_state();
        match P::parse_from_with_rollback_when_no_error(self)? {
            Some(output) => Outcome::Success(output),
            None => f(saved_state).into(),
        }
    }

    fn parse_into<P: ParseFrom<Self>>(mut self) -> Result<(Option<P>, Self), Self::Error> {
        let optional = P::parse_from_with_rollback_when_no_error(&mut self)?;
        Ok((optional, self))
    }
}

pub trait ParseFrom<Context>: Sized
where
    Context: ParseContext + ?Sized,
{
    /// no guarantee on stream state other than Ok(Some(_))
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, Context::Error>;
}
