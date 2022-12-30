use crate::*;

pub trait ParseFromWithRollback<Stream>: ParseFrom<Stream>
where
    Stream: ParseContext + ?Sized,
{
    fn parse_from_with_rollback<'a>(stream: &mut Stream) -> Result<Option<Self>, Self::Error>;
}

impl<Stream, P> ParseFromWithRollback<Stream> for P
where
    Stream: ParseContext + ?Sized,
    P: ParseFrom<Stream>,
{
    fn parse_from_with_rollback<'a>(stream: &mut Stream) -> Result<Option<Self>, Self::Error> {
        let state = stream.save_state();
        let result = Self::parse_from_without_guaranteed_rollback(stream);
        match result {
            // rollback for no pattern
            Ok(None) => stream.rollback(state),
            _ => (),
        }
        result
    }
}
