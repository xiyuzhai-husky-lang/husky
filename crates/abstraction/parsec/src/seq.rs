use crate::*;

pub fn parse_seq2<'a, Context, A, B, Error>(stream: &mut Context) -> Result<Option<(A, B)>, Error>
where
    Context: StreamParser,
    A: TryParseOptionFromStream<Context, Error = Error>,
    B: TryParseOptionFromStream<Context, Error = Error>,
{
    let a = match A::try_parse_option_from_stream_without_guaranteed_rollback(stream)? {
        Some(a) => a,
        None => return Ok(None),
    };
    let b = match B::try_parse_option_from_stream_without_guaranteed_rollback(stream)? {
        Some(b) => b,
        None => return Ok(None),
    };
    Ok(Some((a, b)))
}

#[test]
fn parse_seq2_works() {
    fn t(input: &str) -> Result<Option<(A, B)>, ()> {
        parse_seq2(&mut CharStream::new(input))
    }
    assert_eq!(t("ab"), Ok(Some((A {}, B {}))));
    assert_eq!(t("ba"), Ok(None));
}

pub fn parse_seq3<'a, Context, A, B, C, Error>(
    stream: &mut Context,
) -> Result<Option<(A, B, C)>, Error>
where
    Context: StreamParser,
    A: TryParseOptionFromStream<Context, Error = Error>,
    B: TryParseOptionFromStream<Context, Error = Error>,
    C: TryParseOptionFromStream<Context, Error = Error>,
{
    let a = match A::try_parse_option_from_stream_without_guaranteed_rollback(stream)? {
        Some(a) => a,
        None => return Ok(None),
    };
    let b = match B::try_parse_option_from_stream_without_guaranteed_rollback(stream)? {
        Some(b) => b,
        None => return Ok(None),
    };
    let c = match C::try_parse_option_from_stream_without_guaranteed_rollback(stream)? {
        Some(c) => c,
        None => return Ok(None),
    };
    Ok(Some((a, b, c)))
}

#[test]
fn parse_seq3_works() {
    fn t(input: &str) -> Result<Option<(A, B, Comma)>, ()> {
        parse_seq3(&mut CharStream::new(input))
    }
    assert_eq!(t("ab,"), Ok(Some((A {}, B {}, Comma {}))));
    assert_eq!(t("ab"), Ok(None));
    assert_eq!(t("ba"), Ok(None));
}
