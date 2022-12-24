use crate::*;

pub trait ParseFromWithRollback<Stream>: ParseFrom<Stream>
where
    Stream: ParseInto + ?Sized,
{
    fn parse_from_with_rollback<'a>(
        stream: &mut Stream,
    ) -> Result<Option<Self::Output>, Self::Error>;
}

impl<Stream, P> ParseFromWithRollback<Stream> for P
where
    Stream: ParseInto + ?Sized,
    P: ParseFrom<Stream>,
{
    fn parse_from_with_rollback<'a>(
        stream: &mut Stream,
    ) -> Result<Option<Self::Output>, Self::Error> {
        let clone = stream.clone();
        let result = Self::parse_from(stream);
        match result {
            // rollback for no pattern
            Ok(None) => *stream = clone,
            _ => (),
        }
        result
    }
}
