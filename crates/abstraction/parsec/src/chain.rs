use crate::*;

pub fn parse_seq2_from<'a, Stream, A, B, Error>(
    stream: &mut Stream,
) -> Result<Option<(A, B)>, Error>
where
    Stream: ParseContext,
    A: ParseFrom<Stream>,
    B: ParseFrom<Stream>,
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

pub fn parse_seq3_from<'a, Stream, A, B, C, Error>(
    stream: &mut Stream,
) -> Result<Option<(A, B, C)>, Error>
where
    Stream: ParseContext,
    A: ParseFrom<Stream>,
    B: ParseFrom<Stream>,
    C: ParseFrom<Stream>,
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
