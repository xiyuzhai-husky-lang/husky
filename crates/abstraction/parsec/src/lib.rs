mod chain;
mod rollback;
mod separated_list;
#[cfg(test)]
mod tests;

pub use rollback::*;

pub trait ParseInto: Clone + Sized {
    /// returns an optional and the rest of the stream,
    ///
    /// guarantees that stream state is not changed if result is Ok(None)
    fn parse_into<P: ParseFrom<Self>>(self) -> Result<(Option<P::Output>, Self), P::Error>;
}

impl<T> ParseInto for T
where
    T: Clone + Sized,
{
    fn parse_into<P: ParseFrom<Self>>(mut self) -> Result<(Option<P::Output>, Self), P::Error> {
        let p = P::parse_from_with_rollback(&mut self)?;
        Ok((p, self))
    }
}

pub trait ParseFrom<Stream>: Sized
where
    Stream: ParseInto + ?Sized,
{
    type Output;
    type Error;
    /// no guarantee on stream state other than Ok(Some(_))
    fn parse_from<'a>(stream: &mut Stream) -> Result<Option<Self::Output>, Self::Error>;
}
