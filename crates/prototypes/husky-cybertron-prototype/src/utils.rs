use crate::*;
use husky_text_protocol::char::TextCharIter;

pub struct ClikeTokenizer<'a, LexicalSpecs: IsLexicalSpecs> {
    chars: TextCharIter<'a>,
    _specs: PhantomData<LexicalSpecs>,
}

impl<'a, LexicalSpecs: IsLexicalSpecs> std::ops::Deref for ClikeTokenizer<'a, LexicalSpecs> {
    type Target = TextCharIter<'a>;

    fn deref(&self) -> &Self::Target {
        &self.chars
    }
}

impl<'a, LexicalSpecs: IsLexicalSpecs> std::ops::DerefMut for ClikeTokenizer<'a, LexicalSpecs> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chars
    }
}

pub trait IsLexicalSpecs: Sized {
    type Token: Copy;

    fn parse_numerical(tokenizer: ClikeTokenizer<Self>, c: char) -> Self::Token;
    fn parse_special(tokenizer: ClikeTokenizer<Self>, c: char) -> Self::Token;
    fn newline() -> Self::Token;
    fn indent(indent: usize) -> Option<Self::Token>;
}

impl<'a, LexicalSpecs: IsLexicalSpecs> Iterator for ClikeTokenizer<'a, LexicalSpecs> {
    type Item = LexicalSpecs::Token;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.chars.next()?;
        match c {
            '\n' => Some(LexicalSpecs::newline()),
            c if c.is_alphabetic() || c == '_' => todo!(),
            _ => todo!(),
        }
    }
}
