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
    fn parse<P: ParseFromStreamWithError<Self>>(
        &mut self,
    ) -> Result<Option<P>, <P as ParseFromStreamWithError<Self>>::Error>;
    fn try_parse<P: ParseFromStreamWithError<Self>>(&mut self) -> Option<P>;

    fn parse_expected<T: ParseFromStreamWithError<Self>, E: OriginalError>(
        &mut self,
        f: impl FnOnce(Self::State) -> E,
    ) -> Result<T, E::Error>
    where
        E::Error: From<<T as ParseFromStreamWithError<Self>>::Error>;

    /// returns an optional and the rest of the stream,
    ///
    /// guarantees that stream state is not changed if result is Ok(None)
    fn parse_into<P: ParseFromStreamWithError<Self>>(
        self,
    ) -> Result<(Option<P>, Self), <P as ParseFromStreamWithError<Self>>::Error>
    where
        Self: Sized;
}

impl<SP> StreamParser for SP
where
    SP: HasStreamState,
{
    fn parse<P: ParseFromStreamWithError<Self>>(
        &mut self,
    ) -> Result<Option<P>, <P as ParseFromStreamWithError<Self>>::Error> {
        P::parse_from_with_rollback_when_no_error(self)
    }

    /// deprecated
    fn try_parse<T: ParseFromStreamWithError<Self>>(&mut self) -> Option<T> {
        T::try_parse_from_with_rollback(self)
    }

    fn parse_expected<T: ParseFromStreamWithError<Self>, E: OriginalError>(
        &mut self,
        f: impl FnOnce(Self::State) -> E,
    ) -> Result<T, E::Error>
    where
        E::Error: From<<T as ParseFromStreamWithError<Self>>::Error>,
    {
        let saved_state = self.save_state();
        match T::parse_from_with_rollback_when_no_error(self)? {
            Some(output) => Ok(output),
            None => Err(f(saved_state).into()),
        }
    }

    fn parse_into<T: ParseFromStreamWithError<Self>>(
        mut self,
    ) -> Result<(Option<T>, Self), <T as ParseFromStreamWithError<Self>>::Error> {
        let optional = T::parse_from_with_rollback_when_no_error(&mut self)?;
        Ok((optional, self))
    }
}

pub trait ParseFromStreamWithError<SP>: Sized
where
    SP: StreamParser + ?Sized,
{
    type Error;

    /// no guarantee on stream state other than Ok(Some(_))
    fn parse_from_without_guaranteed_rollback(ctx: &mut SP) -> Result<Option<Self>, Self::Error>;
}

pub trait ParseFromStream<SP>: Sized
where
    SP: StreamParser + ?Sized,
{
    /// no guarantee on stream state other than Ok(Some(_))
    fn parse_from_without_guaranteed_rollback(ctx: &mut SP) -> Option<Self>;
}
