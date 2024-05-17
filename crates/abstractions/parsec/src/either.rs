use crate::*;
use ::either::*;

impl<SP, A, B> TryParseOptionFromStream<SP> for Either<A, B>
where
    A: TryParseOptionFromStream<SP>,
    B: TryParseOptionFromStream<SP>,
    SP: IsStreamParser + ?Sized,
    A::Error: From<B::Error>,
{
    type Error = A::Error;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut SP,
    ) -> Result<Option<Self>, Self::Error> {
        if let Some(a) = sp.try_parse_option::<A>()? {
            Ok(Some(Left(a)))
        } else if let Some(b) = sp.try_parse_option::<B>()? {
            Ok(Some(Right(b)))
        } else {
            Ok(None)
        }
    }
}
