use crate::*;

pub trait ParseFromWithRollback<Input>: ParseFrom<Input>
where
    Input: ParseInput + ?Sized,
{
    fn parse_from_with_rollback<'a>(
        stream: &mut Input::Stream<'a>,
    ) -> Result<Option<Self>, Self::Error>;
}

impl<Input, P> ParseFromWithRollback<Input> for P
where
    Input: ParseInput + ?Sized,
    P: ParseFrom<Input>,
{
    fn parse_from_with_rollback<'a>(
        stream: &mut <Input as ParseInput>::Stream<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        let clone = stream.clone();
        let result = Self::parse_from(stream);
        match result {
            Ok(Some(_)) => (),
            _ => *stream = clone,
        }
        result
    }
}
