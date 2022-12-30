mod chain;
mod rollback;
mod separated_list;
#[cfg(test)]
mod tests;

pub use rollback::*;

pub trait HasParseState {
    type State;
    fn save_state(&self) -> Self::State;
    fn rollback(&mut self, state: Self::State);
}

impl<Wrapper> HasParseState for Wrapper
where
    Wrapper: std::ops::DerefMut,
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
    fn parse<P: ParseFrom<Self>>(&mut self) -> Result<Option<P>, P::Error>;
    fn parse_expected<P: ParseFrom<Self>, Error>(
        &mut self,
        err: impl FnOnce(Self::State) -> Error,
    ) -> Result<P, Error>
    where
        Error: From<P::Error>;

    /// returns an optional and the rest of the stream,
    ///
    /// guarantees that stream state is not changed if result is Ok(None)
    fn parse_into<P: ParseFrom<Self>>(self) -> Result<(Option<P>, Self), P::Error>
    where
        Self: Sized;
}

impl<T> ParseContext for T
where
    T: HasParseState,
{
    fn parse<P: ParseFrom<Self>>(&mut self) -> Result<Option<P>, P::Error> {
        P::parse_from(self)
    }

    fn parse_expected<P: ParseFrom<Self>, Error>(
        &mut self,
        err: impl FnOnce(Self::State) -> Error,
    ) -> Result<P, Error>
    where
        Error: From<P::Error>,
    {
        let saved_state = self.save_state();
        match P::parse_from(self)? {
            Some(output) => Ok(output),
            None => Err(err(saved_state)),
        }
    }

    fn parse_into<P: ParseFrom<Self>>(mut self) -> Result<(Option<P>, Self), P::Error> {
        let optional = P::parse_from_with_rollback(&mut self)?;
        Ok((optional, self))
    }
}

pub trait ParseFrom<Context>: Sized
where
    Context: ParseContext + ?Sized,
{
    type Error;
    /// no guarantee on stream state other than Ok(Some(_))
    fn parse_from<'a>(ctx: &mut Context) -> Result<Option<Self>, Self::Error>;
}
