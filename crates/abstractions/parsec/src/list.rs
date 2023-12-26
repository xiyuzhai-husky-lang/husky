use smallvec::{smallvec, SmallVec};
use vec_like::{AsVecMapEntry, VecMap};

use crate::*;

pub fn parse_consecutive_list<Parser, Element, Error>(
    parser: &mut Parser,
) -> Result<Vec<Element>, Error>
where
    Parser: IsStreamParser,
    Element: TryParseOptionFromStream<Parser, Error = Error>,
{
    let mut elements = vec![];
    while let Some(element) = parser.try_parse_option::<Element>()? {
        elements.push(element)
    }
    Ok(elements)
}

pub fn parse_consecutive_vec_map<Parser, K, Element, Error>(
    parser: &mut Parser,
) -> Result<VecMap<Element>, Error>
where
    Parser: IsStreamParser,
    K: Eq + Copy,
    Element: AsVecMapEntry<K = K> + TryParseOptionFromStream<Parser, Error = Error>,
{
    let mut elements = VecMap::default();
    while let Some(element) = parser.try_parse_option::<Element>()? {
        match elements.insert_new(element) {
            Ok(_) => (),
            Err(_) => todo!(),
        }
    }
    Ok(elements)
}

#[derive(Debug, PartialEq, Eq)]
pub struct PunctuatedSmallList<
    Element,
    Separator,
    Error,
    const ALLOW_TRAILING_SEPARATOR: bool,
    const N: usize,
> where
    [Element; N]: smallvec::Array<Item = Element>,
    [Separator; N]: smallvec::Array<Item = Separator>,
{
    elements: SmallVec<[Element; N]>,
    separators: SmallVec<[Separator; N]>,
    phantom: std::marker::PhantomData<Error>,
}

impl<Element, Separator, Error, const ALLOW_TRAILING_SEPARATOR: bool, const N: usize>
    PunctuatedSmallList<Element, Separator, Error, ALLOW_TRAILING_SEPARATOR, N>
where
    [Element; N]: smallvec::Array<Item = Element>,
    [Separator; N]: smallvec::Array<Item = Separator>,
{
    pub fn elements(&self) -> &[Element] {
        &self.elements
    }

    pub fn separators(&self) -> &[Separator] {
        &self.separators
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

impl<
        StreamParser,
        Element,
        Separator,
        Error,
        const ALLOW_TRAILING_SEPARATOR: bool,
        const N: usize,
    > TryParseFromStream<StreamParser>
    for PunctuatedSmallList<Element, Separator, Error, ALLOW_TRAILING_SEPARATOR, N>
where
    [Element; N]: smallvec::Array<Item = Element>,
    [Separator; N]: smallvec::Array<Item = Separator>,
    StreamParser: IsStreamParser + ?Sized,
    Element: TryParseOptionFromStream<StreamParser>,
    Separator: TryParseOptionFromStream<StreamParser>,
    Error: From<Element::Error> + From<Separator::Error>,
{
    type Error = Error;

    fn try_parse_from_stream(parser: &mut StreamParser) -> Result<Self, Error> {
        let mut elements = smallvec![];
        let mut separators = smallvec![];
        // this will not be used if ALLOW_TRAILING_SEPARATOR is true
        let mut saved_state: Option<StreamParser::State> = None;
        loop {
            match parser.try_parse_option::<Element>() {
                Ok(Some(element)) => {
                    elements.push(element);
                    saved_state = Some(parser.save_state());
                    let Some(separator) = parser.try_parse_option::<Separator>()? else {
                        break;
                    };
                    separators.push(separator)
                }
                Ok(None) => {
                    if !ALLOW_TRAILING_SEPARATOR && let Some(saved_state) = saved_state {
                        parser.rollback(saved_state);
                        separators.pop();
                        debug_assert_eq!(elements.len(), separators.len() + 1);
                    }
                    break;
                }
                Err(error) => return Err(error.into()),
            }
        }
        Ok(Self {
            elements,
            separators,
            phantom: std::marker::PhantomData,
        })
    }
}

pub fn parse_separated_list<SP, Element, Separator, Error>(
    ctx: &mut SP,
) -> (Vec<Element>, Vec<Separator>, Result<(), Error>)
where
    SP: IsStreamParser,
    Element: TryParseOptionFromStream<SP, Error = Error>,
    Separator: TryParseOptionFromStream<SP>,
    Error: From<<Separator as TryParseOptionFromStream<SP>>::Error>,
{
    let mut elements = vec![];
    let mut separators = vec![];
    let result = loop {
        match ctx.try_parse_option::<Element>() {
            Ok(Some(element)) => {
                elements.push(element);
                match ctx.try_parse_option::<Separator>() {
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
    Context: IsStreamParser,
    Element: TryParseOptionFromStream<Context, Error = E1>,
    Separator: TryParseOptionFromStream<Context>,
    E1: From<<Separator as TryParseOptionFromStream<Context>>::Error>,
{
    let mut elements = vec![];
    let mut separators = vec![];
    loop {
        match ctx.try_parse_option::<Element>() {
            Ok(Some(element)) => {
                elements.push(element);
                match ctx.try_parse_option::<Separator>() {
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

pub fn parse_separated_small_list2<Context, Element, Separator, E1, E2>(
    ctx: &mut Context,
    f: impl FnOnce(E1) -> E2,
) -> Result<(SmallVec<[Element; 2]>, SmallVec<[Separator; 2]>), E2>
where
    Context: IsStreamParser,
    Element: TryParseOptionFromStream<Context, Error = E1>,
    Separator: TryParseOptionFromStream<Context>,
    E1: From<<Separator as TryParseOptionFromStream<Context>>::Error>,
{
    let mut elements = smallvec![];
    let mut separators = smallvec![];
    loop {
        match ctx.try_parse_option::<Element>() {
            Ok(Some(element)) => {
                elements.push(element);
                match ctx.try_parse_option::<Separator>() {
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
    Context: IsStreamParser,
    Element: TryParseOptionFromStream<Context>,
    Separator: TryParseOptionFromStream<Context>,
    E::Error: From<<Element as TryParseOptionFromStream<Context>>::Error>,
    E::Error: From<<Separator as TryParseOptionFromStream<Context>>::Error>,
{
    let mut elements = vec![];
    let mut separators = vec![];
    let result = loop {
        let state = ctx.save_state();
        match ctx.try_parse_option::<Element>() {
            Ok(Some(element)) => {
                elements.push(element);
                match ctx.try_parse_option::<Separator>() {
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

pub fn parse_separated_small2_list_expected<Context, Element, Separator, E: OriginalError>(
    ctx: &mut Context,
    nelem_min: usize,
    f: impl FnOnce(<Context as HasStreamState>::State) -> E,
) -> (
    SmallVec<[Element; 2]>,
    SmallVec<[Separator; 2]>,
    Result<(), E::Error>,
)
where
    Context: IsStreamParser,
    Element: TryParseOptionFromStream<Context>,
    Separator: TryParseOptionFromStream<Context>,
    E::Error: From<<Element as TryParseOptionFromStream<Context>>::Error>,
    E::Error: From<<Separator as TryParseOptionFromStream<Context>>::Error>,
{
    let mut elements = smallvec![];
    let mut separators = smallvec![];
    let result = loop {
        let state = ctx.save_state();
        match ctx.try_parse_option::<Element>() {
            Ok(Some(element)) => {
                elements.push(element);
                match ctx.try_parse_option::<Separator>() {
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
