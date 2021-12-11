use std::{iter::Enumerate, num::NonZeroU16, str::Chars};

use common::*;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    start: Column,
    end: Column,
    kind: TokenKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Column(NonZeroU16);
impl From<u32> for Column {
    fn from(raw: u32) -> Self {
        unsafe {
            Column(NonZeroU16::new_unchecked(
                <u32 as TryInto<u16>>::try_into(raw).expect("success") + 1,
            ))
        }
    }
}
impl From<usize> for Column {
    fn from(raw: usize) -> Self {
        unsafe {
            Column(NonZeroU16::new_unchecked(
                <usize as TryInto<u16>>::try_into(raw).expect("success") + 1,
            ))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Keyword(word::Keyword),
    Identifier(word::Identifier),
    Special(Special),
    I32Literal(i32),
    F32Literal(f32),
}
impl Eq for TokenKind {}

struct TokenIter<'lex_line> {
    db: &'lex_line dyn LexQuery,
    buffer: String,
    char_iter: CharIter<'lex_line>,
}

type CharIter<'lex_line> = std::iter::Peekable<Enumerate<Chars<'lex_line>>>;

impl<'lex_line> TokenIter<'lex_line> {
    pub fn new(db: &'lex_line dyn LexQuery, mut char_iter: CharIter<'lex_line>) -> (Indent, Self) {
        let mut buffer = String::new();
        buffer.reserve(100);
        let indent = Indent::from(&mut char_iter);
        (
            indent,
            Self {
                db,
                buffer,
                char_iter,
            },
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Indent(Option<u16>);
impl<'lex_line> From<&mut CharIter<'lex_line>> for Indent {
    fn from(char_iter: &mut CharIter<'lex_line>) -> Self {
        loop {
            if let Some((j, c)) = char_iter.peek() {
                if *c == ' ' {
                    char_iter.next();
                } else {
                    let j: u16 = (*j).try_into().expect("yes");
                    break Indent(Some(j));
                }
            } else {
                break Indent(None);
            }
        }
    }
}

impl<'lex_line> TokenIter<'lex_line> {
    fn skip_whitespaces(&mut self) {
        while let Some((_, c)) = self.char_iter.peek() {
            if *c != ' ' {
                break;
            } else {
                self.char_iter.next();
            }
        }
    }
    fn next_word(&mut self, j_start: usize, c_start: char) -> Token {
        let start = Column::from(j_start);
        self.buffer.push(c_start);
        let end = loop {
            if let Some((j, c)) = self.char_iter.peek() {
                let c = *c;
                if is_word_char(c) {
                    self.buffer.push(c);
                    self.char_iter.next();
                } else {
                    break Column::from(*j);
                }
            } else {
                break start;
            }
        };
        let word = self.db.string_to_word(&self.buffer);
        self.buffer.clear();
        return Token {
            start,
            end,
            kind: match word {
                word::Word::Keyword(keyword) => TokenKind::Keyword(keyword),
                word::Word::Identifier(ident) => TokenKind::Identifier(ident),
            },
        };

        fn is_word_char(c: char) -> bool {
            c.is_alphanumeric() || c == '_'
        }
    }
    fn next_number(&mut self, j_start: usize, c_start: char) -> Token {
        self.buffer.push(c_start);
        while self.peek().is_digit(10) {
            self.eat()
        }
        if self.peek() == '.' {
            self.eat();
            while self.peek().is_digit(10) {
                self.eat()
            }
            let len = self.buffer.len();
            let f = self.buffer.parse::<f32>().expect("couldn't be wrong");
            self.buffer.clear();
            Token {
                start: Column::from(j_start),
                end: Column::from(j_start + len),
                kind: TokenKind::F32Literal(f),
            }
        } else {
            let len = self.buffer.len();
            let i = self.buffer.parse::<i32>().expect("couldn't be wrong");
            self.buffer.clear();
            Token {
                start: Column::from(j_start),
                end: Column::from(j_start + len),
                kind: TokenKind::I32Literal(i),
            }
        }
    }
    fn peek(&mut self) -> char {
        if let Some((_, c)) = self.char_iter.peek() {
            *c
        } else {
            ' '
        }
    }
    fn pass(&mut self) {
        self.char_iter.next();
    }
    fn eat(&mut self) {
        if let Some((_, c)) = self.char_iter.next() {
            self.buffer.push(c);
        }
    }
    fn next_special(&mut self, j_start: usize, c_start: char) -> Option<Token> {
        let (len, special) = match c_start {
            '=' => match self.peek() {
                '=' => {
                    self.pass();
                    (2, Special::Eq)
                }
                '>' => {
                    self.pass();
                    (2, Special::HeavyArrow)
                }
                _ => (1, Special::Be),
            },
            ':' => match self.peek() {
                ':' => {
                    self.pass();
                    (2, Special::ScopeAccess)
                }
                _ => (1, Special::Colon),
            },
            '(' => (1, Special::LPar),
            '[' => (1, Special::LBox),
            '{' => (1, Special::LCurl),
            ')' => (1, Special::RPar),
            ']' => (1, Special::RBox),
            '}' => (1, Special::RCurl),
            ',' => (1, Special::Comma),
            '&' => match self.peek() {
                '&' => {
                    self.pass();
                    (2, Special::And)
                }
                _ => (1, Special::Ambersand),
            },
            '|' => match self.peek() {
                '|' => {
                    self.pass();
                    (2, Special::Or)
                }
                _ => (1, Special::Vertical),
            },
            '~' => (1, Special::BitNot),
            '.' => (1, Special::MemberAccess),
            '%' => (1, Special::Modulo),
            '-' => match self.peek() {
                '=' => {
                    self.pass();
                    (2, Special::SubAssign)
                }
                '-' => {
                    self.pass();
                    (2, Special::Decr)
                }
                _ => (1, Special::Sub),
            },
            '<' => match self.peek() {
                '<' => {
                    self.pass();
                    (2, Special::RShift)
                }
                '=' => {
                    self.pass();
                    (2, Special::Leq)
                }
                _ => (1, Special::LessOrLAngular),
            },
            '>' => {
                match self.peek() {
                    '>' => {
                        // >>
                        self.pass();
                        (2, Special::LShift)
                    }
                    '=' => {
                        // >=
                        self.pass();
                        (2, Special::Geq)
                    }
                    _ => (1, Special::GreaterOrRAngular),
                }
            }
            '*' => {
                match self.peek() {
                    '*' => {
                        // <<
                        self.pass();
                        todo!()
                    }
                    '=' => {
                        // <<
                        self.pass();
                        todo!()
                    }
                    _ => (1, Special::Mult),
                }
            }
            '/' => {
                match self.peek() {
                    '/' => {
                        return None;
                    }
                    '=' => {
                        // <<
                        self.pass();
                        todo!()
                    }
                    _ => (1, Special::Div),
                }
            }
            '+' => match self.peek() {
                '+' => {
                    self.pass();
                    (2, Special::Incr)
                }
                '=' => {
                    self.pass();
                    todo!()
                }
                _ => (1, Special::Add),
            },
            '!' => match self.peek() {
                '=' => {
                    self.pass();
                    (2, Special::Neq)
                }
                _ => (1, Special::NotOrExclusive),
            },
            _ => todo!(),
        };
        Some(Token {
            start: Column::from(j_start),
            end: Column::from(j_start + len),
            kind: TokenKind::Special(special),
        })
    }
}

impl<'lex_line> Iterator for TokenIter<'lex_line> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((j, c)) = self.char_iter.next() {
            if c == ' ' {
                self.skip_whitespaces();
                return self.next();
            } else if c.is_alphabetic() || c == '_' {
                Some(self.next_word(j, c))
            } else if c.is_digit(10) {
                Some(self.next_number(j, c))
            } else {
                self.next_special(j, c)
            }
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenizedLine {
    pub(crate) indent: Indent,
    pub(crate) start: TokenIndex,
    pub(crate) end: TokenIndex,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenIndex(u32);

impl From<usize> for TokenIndex {
    fn from(raw: usize) -> Self {
        Self(raw.try_into().expect("Token index shouldn't overflow u32"))
    }
}

#[derive(Debug)]
pub(crate) struct TokenScanner<'lex> {
    db: &'lex dyn LexQuery,
    tokens: Vec<Token>,
    tokenized_lines: Vec<TokenizedLine>,
}

impl<'lex> TokenScanner<'lex> {
    pub(crate) fn new(db: &'lex dyn LexQuery) -> Self {
        Self {
            db,
            tokens: vec![],
            tokenized_lines: vec![],
        }
    }
    pub(crate) fn scan(&mut self, line: &str) {
        let start = TokenIndex::from(self.tokens.len());
        let (indent, token_iter) = TokenIter::new(self.db, line.chars().enumerate().peekable());
        self.tokens.extend(token_iter);
        let end = TokenIndex::from(self.tokens.len());
        self.tokenized_lines
            .push(TokenizedLine { indent, start, end })
    }
}

impl<'lex> Into<TokenizedText> for TokenScanner<'lex> {
    fn into(self) -> TokenizedText {
        TokenizedText {
            tokens: self.tokens,
            tokenized_lines: self.tokenized_lines,
        }
    }
}
