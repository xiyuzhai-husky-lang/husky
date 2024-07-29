pub mod ident;
pub mod keyword;
pub mod literal;
pub mod opr;

use self::{ident::Ident, keyword::Keyword, literal::Literal, opr::Opr};
use crate::*;
use husky_cybertron::seq::Seq;
use husky_text_protocol::char::TextCharIter;

#[enum_class::from_variants]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Literal(Literal),
    Keyword(Keyword),
    Ident(Ident),
    Opr(Opr),
}

pub enum Convexity {
    Convex,
    Concave,
}

impl Token {
    pub fn right_convexity(self) -> Convexity {
        match self {
            Token::Literal(_) => Convexity::Convex,
            Token::Keyword(_) => Convexity::Concave,
            Token::Ident(_) => Convexity::Convex,
            Token::Opr(opr) => match opr {
                Opr::Prefix(_) => Convexity::Concave,
                Opr::Binary(_) => Convexity::Concave,
                Opr::Suffix(_) => Convexity::Convex,
            },
        }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Literal(lit) => write!(f, "`{}`", lit),
            Token::Keyword(kw) => write!(f, "`{}`", kw.data()),
            Token::Ident(ident) => write!(f, "`{}`", ident.data()),
            Token::Opr(opr) => write!(f, "`{}`", opr.data()),
        }
    }
}

pub fn tokenize(input: &str) -> Seq<Token> {
    Seq::new(Tokenizer::new(input).collect())
}

struct Tokenizer<'a> {
    chars: TextCharIter<'a>,
    last_token_right_convexity: Convexity,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: TextCharIter::new(input),
            last_token_right_convexity: Convexity::Concave,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_aux();
        if let Some(token) = token {
            self.last_token_right_convexity = token.right_convexity();
        }
        token
    }
}

/// # actions
impl<'a> Tokenizer<'a> {
    fn next_aux(&mut self) -> Option<Token> {
        match self.chars.next()? {
            ' ' => self.next(),
            '+' => Some(Opr::ADD.into()),
            '-' => match self.last_token_right_convexity {
                Convexity::Convex => Some(Opr::SUB.into()),
                Convexity::Concave => Some(Opr::MINUS.into()),
            },
            '*' => Some(Opr::MUL.into()),
            '/' => Some(Opr::DIV.into()),
            '=' => Some(Opr::ASSIGN.into()),
            c if c.is_alphabetic() || c == '_' => Some(self.next_keyword_or_ident(c)),
            c if c.is_numeric() => Some(self.next_numeric_literal(c)),
            c => todo!(),
        }
    }

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

    fn next_numeric_literal(&mut self, c: char) -> Token {
        let mut s = String::from(c);
        s += self.chars.next_str_slice_while(|c| c.is_numeric());
        let i: i32 = s.parse().unwrap();
        Token::Literal(Literal::Int(i))
    }
}

#[test]
fn tokenize_works() {
    fn t(input: &str, expect: Expect) {
        expect.assert_debug_eq(&tokenize(input));
    }
    t(
        "hello",
        expect![[r#"
        [`hello`]
    "#]],
    );
    t(
        "hello ",
        expect![[r#"
        [`hello`]
    "#]],
    );
    t(
        " hello ",
        expect![[r#"
        [`hello`]
    "#]],
    );
    t(
        " let hello = world",
        expect![[r#"
            [`let`, `hello`, `=`, `world`]
        "#]],
    );
    t(
        " let hello = 1",
        expect![[r#"
            [`let`, `hello`, `=`, `1`]
        "#]],
    );
    t(
        " let hello = world + 1",
        expect![[r#"
            [`let`, `hello`, `=`, `world`, `+`, `1`]
        "#]],
    );
    t(
        "-1",
        expect![[r#"
            [`-(minus)`, `1`]
        "#]],
    );
    t(
        "1+-1",
        expect![[r#"
            [`1`, `+`, `-(minus)`, `1`]
        "#]],
    );
    t(
        "1-1",
        expect![[r#"
            [`1`, `-(sub)`, `1`]
        "#]],
    );
}
