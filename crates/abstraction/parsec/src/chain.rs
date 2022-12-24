use crate::*;

pub fn parse_seq2_from<'a, Input, A, B, Error>(
    stream: &mut Input::Stream<'a>,
) -> Result<Option<(A, B)>, Error>
where
    Input: ParseInput,
    A: ParseFrom<Input>,
    B: ParseFrom<Input>,
    Error: From<A::Error> + From<B::Error>,
{
    let a = match A::parse_from(stream)? {
        Some(a) => a,
        None => return Ok(None),
    };
    let b = match B::parse_from(stream)? {
        Some(b) => b,
        None => return Ok(None),
    };
    Ok(Some((a, b)))
}

pub fn parse_seq3_from<'a, Input, A, B, C, Error>(
    stream: &mut Input::Stream<'a>,
) -> Result<Option<(A, B, C)>, Error>
where
    Input: ParseInput,
    A: ParseFrom<Input>,
    B: ParseFrom<Input>,
    C: ParseFrom<Input>,
    Error: From<A::Error> + From<B::Error> + From<C::Error>,
{
    let a = match A::parse_from(stream)? {
        Some(a) => a,
        None => return Ok(None),
    };
    let b = match B::parse_from(stream)? {
        Some(b) => b,
        None => return Ok(None),
    };
    let c = match C::parse_from(stream)? {
        Some(c) => c,
        None => return Ok(None),
    };
    Ok(Some((a, b, c)))
}
