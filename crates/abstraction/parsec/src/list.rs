use crate::*;

pub fn parse_consecutive_list<Context, Element, Error>(
    ctx: &mut Context,
) -> Result<Vec<Element>, Error>
where
    Context: Parser,
    Element: ParseFrom<Context, Error = Error>,
{
    let mut elements = vec![];
    while let Some(element) = ctx.parse::<Element>()? {
        elements.push(element)
    }
    Ok(elements)
}

pub fn parse_separated_list<Context, Element, Separator, Error>(
    ctx: &mut Context,
) -> (Vec<Element>, Vec<Separator>, Result<(), Error>)
where
    Context: Parser,
    Element: ParseFrom<Context, Error = Error>,
    Separator: ParseFrom<Context>,
    Error: From<<Separator as ParseFrom<Context>>::Error>,
{
    let mut elements = vec![];
    let mut separators = vec![];
    let result = loop {
        match ctx.parse::<Element>() {
            Ok(Some(element)) => {
                elements.push(element);
                match ctx.parse::<Separator>() {
                    Ok(Some(separator)) => separators.push(separator),
                    Ok(None) => break Ok(()),
                    Err(error) => break Err(error.into()),
                }
            }
            Ok(None) => break Ok(()),
            Err(error) => break Err(error),
        }
    };
    (elements, separators, result)
}

pub fn parse_separated_list2<Context, Element, Separator, E1, E2>(
    ctx: &mut Context,
    f: impl FnOnce(E1) -> E2,
) -> Result<(Vec<Element>, Vec<Separator>), E2>
where
    Context: Parser,
    Element: ParseFrom<Context, Error = E1>,
    Separator: ParseFrom<Context>,
    E1: From<<Separator as ParseFrom<Context>>::Error>,
{
    let mut elements = vec![];
    let mut separators = vec![];
    loop {
        match ctx.parse::<Element>() {
            Ok(Some(element)) => {
                elements.push(element);
                match ctx.parse::<Separator>() {
                    Ok(Some(separator)) => separators.push(separator),
                    Ok(None) => break,
                    Err(error) => return Err(f(error.into())),
                }
            }
            Ok(None) => break,
            Err(error) => return Err(f(error)),
        }
    }
    Ok((elements, separators))
}

#[test]
fn parse_separated_list_works() {
    fn t(input: &str) -> (Vec<A>, Vec<Comma>, Result<(), ()>) {
        parse_separated_list(&mut CharStream::new(input))
    }
    assert_eq!(t("a,a"), (vec![A {}, A {}], vec![Comma {}], Ok(())));
    assert_eq!(t("a,ab"), (vec![A {}, A {}], vec![Comma {}], Ok(())));
    assert_eq!(t("a,bab"), (vec![A {},], vec![Comma {}], Ok(())));
    assert_eq!(
        t("a,a,"),
        (vec![A {}, A {}], vec![Comma {}, Comma {}], Ok(()))
    );
}

pub fn parse_separated_list_expected<Context, Element, Separator, E: OriginalError>(
    ctx: &mut Context,
    nelem_min: usize,
    f: impl FnOnce(<Context as HasParseState>::State) -> E,
) -> (Vec<Element>, Vec<Separator>, Result<(), E::Error>)
where
    Context: Parser,
    Element: ParseFrom<Context>,
    Separator: ParseFrom<Context>,
    E::Error: From<<Element as ParseFrom<Context>>::Error>,
    E::Error: From<<Separator as ParseFrom<Context>>::Error>,
{
    let mut elements = vec![];
    let mut separators = vec![];
    let result = loop {
        let state = ctx.save_state();
        match ctx.parse::<Element>() {
            Ok(Some(element)) => {
                elements.push(element);
                match ctx.parse::<Separator>() {
                    Ok(Some(separator)) => separators.push(separator),
                    Ok(None) => {
                        if elements.len() < nelem_min {
                            break Err(f(state).into());
                        } else {
                            break Ok(());
                        }
                    }
                    Err(error) => break Err(error.into()),
                }
            }
            Ok(None) => {
                if elements.len() < nelem_min {
                    break Err(f(state).into());
                } else {
                    break Ok(());
                }
            }
            Err(error) => break Err(error.into()),
        }
    };
    assert!(result.is_err() || elements.len() >= nelem_min);
    (elements, separators, result)
}

#[test]
fn parse_separated_list_expected_works() {
    fn t(input: &str, nelem_min: usize) -> (Vec<A>, Vec<Comma>, Result<(), ()>) {
        parse_separated_list_expected(&mut CharStream::new(input), nelem_min, |_| ())
    }
    assert_eq!(t("a,a", 0), (vec![A {}, A {}], vec![Comma {}], Ok(())));
    assert_eq!(t("a,a", 1,), (vec![A {}, A {}], vec![Comma {}], Ok(())));
    assert_eq!(t("a,a", 2), (vec![A {}, A {}], vec![Comma {}], Ok(())));
    assert_eq!(t("a,a", 3), (vec![A {}, A {}], vec![Comma {}], Err(())));
    assert_eq!(t("a,ab", 0), (vec![A {}, A {}], vec![Comma {}], Ok(())));
    assert_eq!(t("a,ab", 1), (vec![A {}, A {}], vec![Comma {}], Ok(())));
    assert_eq!(t("a,ab", 2), (vec![A {}, A {}], vec![Comma {}], Ok(())));
    assert_eq!(t("a,ab", 3), (vec![A {}, A {}], vec![Comma {}], Err(())));
    assert_eq!(t("a,bab", 0), (vec![A {},], vec![Comma {}], Ok(())));
    assert_eq!(t("a,bab", 1), (vec![A {},], vec![Comma {}], Ok(())));
    assert_eq!(t("a,bab", 2), (vec![A {},], vec![Comma {}], Err(())));
    assert_eq!(
        t("a,a,", 0),
        (vec![A {}, A {}], vec![Comma {}, Comma {}], Ok(()))
    );
    assert_eq!(
        t("a,a,", 1),
        (vec![A {}, A {}], vec![Comma {}, Comma {}], Ok(()))
    );
    assert_eq!(
        t("a,a,", 2),
        (vec![A {}, A {}], vec![Comma {}, Comma {}], Ok(()))
    );
    assert_eq!(
        t("a,a,", 3),
        (vec![A {}, A {}], vec![Comma {}, Comma {}], Err(()))
    );
}
