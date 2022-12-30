use crate::*;

pub fn parse_seq2<'a, Context, A, B>(stream: &mut Context) -> Result<Option<(A, B)>, Context::Error>
where
    Context: ParseContext,
    A: ParseFrom<Context>,
    B: ParseFrom<Context>,
{
    let a = match A::parse_from_without_guaranteed_rollback(stream)? {
        Some(a) => a,
        None => return Ok(None),
    };
    let b = match B::parse_from_without_guaranteed_rollback(stream)? {
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

pub fn parse_seq3<'a, Context, A, B, C>(
    stream: &mut Context,
) -> Result<Option<(A, B, C)>, Context::Error>
where
    Context: ParseContext,
    A: ParseFrom<Context>,
    B: ParseFrom<Context>,
    C: ParseFrom<Context>,
{
    let a = match A::parse_from_without_guaranteed_rollback(stream)? {
        Some(a) => a,
        None => return Ok(None),
    };
    let b = match B::parse_from_without_guaranteed_rollback(stream)? {
        Some(b) => b,
        None => return Ok(None),
    };
    let c = match C::parse_from_without_guaranteed_rollback(stream)? {
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
