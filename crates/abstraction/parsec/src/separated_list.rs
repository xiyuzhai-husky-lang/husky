use crate::*;

pub fn parse_separated_list<Stream, Element, Separator, Error>(
    stream: &mut Stream,
) -> Result<Vec<Element::Output>, Error>
where
    Stream: ParseContext,
    Element: ParseFrom<Stream>,
    Separator: ParseFrom<Stream>,
    Error: From<Element::Error> + From<Separator::Error>,
{
    todo!()
}
