use vec_like::{AsVecMapEntry, VecMap};

use crate::*;

pub fn parse_consecutive_list<Parser, Element, Error>(
    parser: &mut Parser,
) -> Result<Vec<Element>, Error>
where
    Parser: StreamParser,
    Element: TryParseOptionalFromStream<Parser, Error = Error>,
{
    let mut elements = vec![];
    while let Some(element) = parser.try_parse_optional::<Element>()? {
        elements.push(element)
    }
    Ok(elements)
}

pub fn parse_consecutive_vec_map<Parser, K, Element, Error>(
    parser: &mut Parser,
) -> Result<VecMap<Element>, Error>
where
    Parser: StreamParser,
    K: Eq + Copy,
    Element: AsVecMapEntry<K = K> + TryParseOptionalFromStream<Parser, Error = Error>,
{
    let mut elements = VecMap::default();
    while let Some(element) = parser.try_parse_optional::<Element>()? {
        match elements.insert_new(element) {
            Ok(_) => (),
            Err(_) => todo!(),
        }
    }
    Ok(elements)
}

#[derive(Debug, PartialEq, Eq)]
pub struct SeparatedListWithKet<Element, Separator, Ket, Error> {
    elements: Vec<Element>,
    separators: Vec<Separator>,
    ket: Ket,
    result: Result<(), Error>,
}

impl<SP, Element, Separator, Ket, Error> TryParseFromStream<SP>
    for SeparatedListWithKet<Element, Separator, Ket, Error>
where
    SP: StreamParser + ?Sized,
{
    type Error = Error;

    fn try_parse_from_without_guaranteed_rollback(sp: &mut SP) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub fn parse_separated_list<Context, Element, Separator, Error>(
    ctx: &mut Context,
) -> (Vec<Element>, Vec<Separator>, Result<(), Error>)
where
    Context: StreamParser,
    Element: TryParseOptionalFromStream<Context, Error = Error>,
    Separator: TryParseOptionalFromStream<Context>,
    Error: From<<Separator as TryParseOptionalFromStream<Context>>::Error>,
{
    let mut elements = vec![];
    let mut separators = vec![];
    let result = loop {
        match ctx.try_parse_optional::<Element>() {
            Ok(Some(element)) => {
                elements.push(element);
                match ctx.try_parse_optional::<Separator>() {
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
    Context: StreamParser,
    Element: TryParseOptionalFromStream<Context, Error = E1>,
    Separator: TryParseOptionalFromStream<Context>,
    E1: From<<Separator as TryParseOptionalFromStream<Context>>::Error>,
{
    let mut elements = vec![];
    let mut separators = vec![];
    loop {
        match ctx.try_parse_optional::<Element>() {
            Ok(Some(element)) => {
                elements.push(element);
                match ctx.try_parse_optional::<Separator>() {
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
    f: impl FnOnce(<Context as HasStreamState>::State) -> E,
) -> (Vec<Element>, Vec<Separator>, Result<(), E::Error>)
where
    Context: StreamParser,
    Element: TryParseOptionalFromStream<Context>,
    Separator: TryParseOptionalFromStream<Context>,
    E::Error: From<<Element as TryParseOptionalFromStream<Context>>::Error>,
    E::Error: From<<Separator as TryParseOptionalFromStream<Context>>::Error>,
{
    let mut elements = vec![];
    let mut separators = vec![];
    let result = loop {
        let state = ctx.save_state();
        match ctx.try_parse_optional::<Element>() {
            Ok(Some(element)) => {
                elements.push(element);
                match ctx.try_parse_optional::<Separator>() {
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
