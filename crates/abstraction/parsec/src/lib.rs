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

pub trait ParseInto: HasParseState {
    /// returns an optional and the rest of the stream,
    ///
    /// guarantees that stream state is not changed if result is Ok(None)
    fn parse_into<P: ParseFrom<Self>>(self) -> Result<(Option<P::Output>, Self), P::Error>
    where
        Self: Sized;
}

impl<T> ParseInto for T
where
    T: HasParseState,
{
    fn parse_into<P: ParseFrom<Self>>(mut self) -> Result<(Option<P::Output>, Self), P::Error> {
        let optional = P::parse_from_with_rollback(&mut self)?;
        Ok((optional, self))
    }
}

pub trait ParseFrom<ParseContext>: Sized
where
    ParseContext: ParseInto + ?Sized,
{
    type Output;
    type Error;
    /// no guarantee on stream state other than Ok(Some(_))
    fn parse_from<'a>(ctx: &mut ParseContext) -> Result<Option<Self::Output>, Self::Error>;
}
