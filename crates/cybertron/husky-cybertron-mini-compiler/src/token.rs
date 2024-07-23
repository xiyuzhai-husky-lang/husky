pub mod ident;
pub mod keyword;
pub mod literal;
pub mod opr;

use self::{ident::Ident, keyword::Keyword, opr::Opr};
use crate::*;
use husky_cybertron::seq::Seq;
use husky_text_protocol::char::TextCharIter;

#[enum_class::from_variants]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Keyword(Keyword),
    Ident(Ident),
    Opr(Opr),
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(kw) => write!(f, "k`{}`", kw.data()),
            Token::Ident(ident) => write!(f, "i`{}`", ident.data()),
            Token::Opr(opr) => write!(f, "o`{}`", opr.data()),
        }
    }
}

pub fn tokenize(input: &str) -> Seq<Token> {
    Seq::new(Tokenizer::new(input).collect())
}

struct Tokenizer<'a> {
    chars: TextCharIter<'a>,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: TextCharIter::new(input),
        }
    }
}

/// # actions
impl<'a> Tokenizer<'a> {
    fn next_keyword_or_ident(&mut self, c: char) -> Token {
        let mut s = String::from(c);
        s += self
            .chars
            .next_str_slice_while(|c| c.is_alphanumeric() || c == '_');
        if let Some(keyword) = Keyword::new(&s) {
            return keyword.into();
        }
        Ident::new(s).into()
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next()? {
            ' ' => self.next(),
            '+' => Some(Opr::ADD.into()),
            '-' => Some(Opr::SUB.into()),
            '*' => Some(Opr::MUL.into()),
            '/' => Some(Opr::DIV.into()),
            '=' => Some(Opr::ASSIGN.into()),
            c if c.is_alphabetic() || c == '_' => Some(self.next_keyword_or_ident(c)),
            c => todo!(),
        }
    }
}

#[test]
fn tokenize_works() {
    expect![[r#"
        [i`hello`]
    "#]]
    .assert_debug_eq(&tokenize("hello"));
    expect![[r#"
        [i`hello`]
    "#]]
    .assert_debug_eq(&tokenize("hello "));
    expect![[r#"
        [i`hello`]
    "#]]
    .assert_debug_eq(&tokenize(" hello "));
    expect![[r#"
        [k`let`, i`hello`, o`=`, i`world`]
    "#]]
    .assert_debug_eq(&tokenize(" let hello = world"));
}
