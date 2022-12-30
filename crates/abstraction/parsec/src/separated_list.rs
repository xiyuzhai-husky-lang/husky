use crate::*;

pub fn parse_separated_list<Context, Element, Separator>(
    stream: &mut Context,
) -> Result<(Vec<Element>, Vec<Separator>), Context::Error>
where
    Context: ParseContext,
    Element: ParseFrom<Context>,
    Separator: ParseFrom<Context>,
{
    let mut elements = vec![];
    let mut separators = vec![];
    while let Some(element) = stream.parse::<Element>()? {
        elements.push(element);
        match stream.parse::<Separator>()? {
            Some(separator) => separators.push(separator),
            None => break,
        }
    }
    Ok((elements, separators))
}

#[test]
fn parse_separated_list_works() {
    fn t(input: &str) -> Result<(Vec<A>, Vec<Comma>), ()> {
        parse_separated_list(&mut CharStream::new(input))
    }
    assert_eq!(t("a,a"), Ok((vec![A {}, A {}], vec![Comma {}])));
    assert_eq!(t("a,ab"), Ok((vec![A {}, A {}], vec![Comma {}])));
    assert_eq!(t("a,bab"), Ok((vec![A {},], vec![Comma {}])));
    assert_eq!(t("a,a,"), Ok((vec![A {}, A {}], vec![Comma {}, Comma {}])));
}
