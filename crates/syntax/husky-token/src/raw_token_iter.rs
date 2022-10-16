use std::str::FromStr;

use husky_text::CharIter;
use husky_word::WordInterner;

use crate::*;

pub(crate) struct RawToken {
    pub(crate) range: TextRange,
    pub(crate) kind: RawTokenKind,
}

impl RawToken {
    fn new(i: usize, start: usize, end: usize, kind: RawTokenKind) -> Self {
        RawToken {
            range: husky_text::new_same_line(i, start, end),
            kind,
        }
    }
}

pub enum RawTokenKind {
    Certain(TokenKind),
    Literal(RawLiteralData),
    IllFormedLiteral(RawLiteralData),
    SubOrMinus,
}

impl From<TokenKind> for RawTokenKind {
    fn from(kind: TokenKind) -> Self {
        RawTokenKind::Certain(kind)
    }
}

impl From<Token> for RawToken {
    fn from(value: Token) -> Self {
        Self {
            range: value.range,
            kind: RawTokenKind::Certain(value.kind),
        }
    }
}

pub(crate) struct RawTokenIter<'token_line, 'lex: 'token_line> {
    word_interner: &'lex WordInterner,
    line_index: usize,
    buffer: String,
    char_iter: CharIter<'token_line>,
}

impl<'token_line, 'lex: 'token_line> RawTokenIter<'token_line, 'lex> {
    pub fn new(
        word_interner: &'lex WordInterner,
        line_index: usize,
        mut char_iter: CharIter<'token_line>,
    ) -> (TextIndent, Self) {
        let mut buffer = String::new();
        buffer.reserve_exact(100);
        let indent = TextIndent::from(&mut char_iter);
        (
            indent,
            Self {
                word_interner,
                line_index,
                buffer,
                char_iter,
            },
        )
    }
}

impl<'token_line, 'lex: 'token_line> RawTokenIter<'token_line, 'lex> {
    fn skip_whitespaces(&mut self) {
        while let Some((_, c)) = self.char_iter.peek() {
            if *c != ' ' {
                break;
            } else {
                self.char_iter.next();
            }
        }
    }

    fn next_word(&mut self, j_start: usize) -> Token {
        while let Some((_, c)) = self.char_iter.peek() {
            if is_word_char(*c) {
                self.eat_char();
            } else {
                break;
            }
        }
        let len = self.buffer.len();
        return Token::new(
            self.line_index,
            j_start,
            j_start + len,
            self.take_buffer_word().into(),
        );
    }

    fn next_number(&mut self, j_start: usize) -> RawToken {
        while self.peek_char().is_digit(10) {
            self.eat_char()
        }
        match self.peek_char() {
            '.' => {
                self.eat_char();
                while self.peek_char().is_digit(10) {
                    self.eat_char()
                }
                let len = self.buffer.len();
                RawToken::new(
                    self.line_index,
                    j_start,
                    j_start + len,
                    RawTokenKind::Literal(RawLiteralData::Float(self.take_buffer::<f64>().into())),
                )
            }
            'b' => {
                // b32 or b64
                self.ignore_char();
                let (token_len, kind) = match self.peek_char() {
                    '3' => {
                        self.ignore_char();
                        if self.peek_char() != '2' {
                            (
                                self.buffer.len() + 2,
                                RawTokenKind::IllFormedLiteral(RawLiteralData::Bits(
                                    self.take_buffer::<u64>().into(),
                                )),
                            )
                        } else {
                            // b32
                            self.ignore_char();
                            if is_word_char(self.peek_char()) {
                                todo!()
                            } else {
                                (
                                    self.buffer.len() + 3,
                                    RawTokenKind::Literal(RawLiteralData::B32(
                                        self.take_buffer::<u32>().into(),
                                    )),
                                )
                            }
                        }
                    }
                    '6' => {
                        self.ignore_char();
                        if self.peek_char() != '4' {
                            (
                                self.buffer.len() + 2,
                                RawTokenKind::IllFormedLiteral(RawLiteralData::Bits(
                                    self.take_buffer::<u64>().into(),
                                )),
                            )
                        } else {
                            // b64
                            self.ignore_char();
                            if is_word_char(self.peek_char()) {
                                todo!()
                            } else {
                                (
                                    self.buffer.len() + 3,
                                    RawTokenKind::Literal(RawLiteralData::B64(
                                        self.take_buffer::<u64>().into(),
                                    )),
                                )
                            }
                        }
                    }
                    _ => (
                        self.buffer.len() + 1,
                        RawTokenKind::IllFormedLiteral(RawLiteralData::B64(
                            self.take_buffer::<u64>(),
                        )),
                    ),
                };
                RawToken::new(self.line_index, j_start, j_start + token_len, kind)
            }
            'i' => {
                // i32 or i64
                self.ignore_char();
                let (token_len, kind) = match self.peek_char() {
                    '3' => {
                        self.ignore_char();
                        if self.peek_char() != '2' {
                            (
                                self.buffer.len() + 2,
                                RawTokenKind::IllFormedLiteral(RawLiteralData::Integer(
                                    self.take_buffer::<i32>().into(),
                                )),
                            )
                        } else {
                            // i32
                            self.ignore_char();
                            if is_word_char(self.peek_char()) {
                                todo!()
                            } else {
                                (
                                    self.buffer.len() + 3,
                                    RawTokenKind::Literal(RawLiteralData::I32(
                                        self.take_buffer::<i32>().into(),
                                    )),
                                )
                            }
                        }
                    }
                    '6' => {
                        self.ignore_char();
                        if self.peek_char() != '4' {
                            (
                                self.buffer.len() + 2,
                                RawTokenKind::IllFormedLiteral(RawLiteralData::Integer(
                                    self.take_buffer::<i64>().into(),
                                )),
                            )
                        } else {
                            // b64
                            self.ignore_char();
                            if is_word_char(self.peek_char()) {
                                todo!()
                            } else {
                                (
                                    self.buffer.len() + 3,
                                    RawTokenKind::Literal(RawLiteralData::I64(
                                        self.take_buffer::<i64>().into(),
                                    )),
                                )
                            }
                        }
                    }
                    _ => (
                        self.buffer.len() + 1,
                        RawTokenKind::IllFormedLiteral(RawLiteralData::I64(
                            self.take_buffer::<i64>(),
                        )),
                    ),
                };
                RawToken::new(self.line_index, j_start, j_start + token_len, kind)
            }
            default => {
                if default.is_alphabetic() {
                    // letter other than 'b' or 'i' after integer literal is not allowed
                    let mut token_len = self.buffer.len() + 1;
                    while self.peek_char().is_alphabetic() {
                        self.ignore_char();
                        token_len += 1;
                    }
                    RawToken::new(
                        self.line_index,
                        j_start,
                        j_start + token_len,
                        RawTokenKind::IllFormedLiteral(RawLiteralData::B64(
                            self.take_buffer::<u64>().into(),
                        )),
                    )
                } else {
                    // integer
                    let len = self.buffer.len();
                    RawToken::new(
                        self.line_index,
                        j_start,
                        j_start + len,
                        RawTokenKind::Literal(RawLiteralData::Integer(
                            self.take_buffer::<i32>().into(),
                        )),
                    )
                }
            }
        }
    }

    fn take_buffer_word(&mut self) -> husky_word::WordPtr {
        let word = self.word_interner.intern(std::mem::take(&mut self.buffer));
        self.buffer.clear();
        word
    }
    fn take_buffer<T>(&mut self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        std::mem::take(&mut self.buffer).parse::<T>().unwrap()
    }

    fn peek_char(&mut self) -> char {
        if let Some((_, c)) = self.char_iter.peek() {
            *c
        } else {
            0.into()
        }
    }

    fn pass_two(&mut self, special: SpecialToken) -> (usize, SpecialToken) {
        self.char_iter.next();
        (2, special)
    }

    fn eat_char(&mut self) {
        let (_, c) = self.char_iter.next().expect("what");
        self.buffer.push(c);
    }

    fn ignore_char(&mut self) {
        let (_, _c) = self.char_iter.next().expect("what");
    }

    fn next_special(&mut self, j_start: usize, c_start: char) -> Option<RawToken> {
        let (len, kind) = self.next_special_aux(j_start, c_start)?;
        Some(RawToken::new(self.line_index, j_start, j_start + len, kind).into())
    }

    fn next_special_aux(&mut self, j_start: usize, c_start: char) -> Option<(usize, RawTokenKind)> {
        let (len, special) = match c_start {
            '=' => match self.peek_char() {
                '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                    PureBinaryOpr::Eq,
                )))),
                _ => (1, SpecialToken::BinaryOpr(BinaryOpr::Assign(None))),
            },
            ':' => match self.peek_char() {
                '=' => self.pass_two(SpecialToken::DeriveAssign),
                ':' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::ScopeResolution)),
                _ => (1, SpecialToken::Colon),
            },
            '(' => (1, SpecialToken::Bra(Bracket::Par)),
            '[' => (1, SpecialToken::Bra(Bracket::Box)),
            '{' => (1, SpecialToken::Bra(Bracket::Curl)),
            ')' => (1, SpecialToken::Ket(Bracket::Par)),
            ']' => (1, SpecialToken::Ket(Bracket::Box)),
            '}' => (1, SpecialToken::Ket(Bracket::Curl)),
            ',' => (1, SpecialToken::Comma),
            '@' => (1, SpecialToken::At),
            '&' => match self.peek_char() {
                '&' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::And))),
                '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                    PureBinaryOpr::BitAnd,
                )))),
                _ => (1, SpecialToken::Ambersand),
            },
            '|' => match self.peek_char() {
                '|' => self.pass_two(SpecialToken::DoubleVertical),
                '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                    PureBinaryOpr::BitOr,
                )))),
                _ => (1, SpecialToken::Vertical),
            },
            '~' => (1, SpecialToken::BitNot),
            '.' => (1, SpecialToken::FieldAccess),
            ';' => (1, SpecialToken::Semicolon),
            '%' => (
                1,
                SpecialToken::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::RemEuclid)),
            ),
            '-' => match self.peek_char() {
                '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                    PureBinaryOpr::Sub,
                )))),
                '-' => self.pass_two(SpecialToken::Decr),
                '>' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Curry)),
                _ => return Some((1, RawTokenKind::SubOrMinus)),
            },
            '<' => match self.peek_char() {
                '<' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                    PureBinaryOpr::Shl,
                )))),
                '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::Leq))),
                _ => (1, SpecialToken::LAngle),
            },
            '>' => match self.peek_char() {
                // '>' => self.pass_two(SpecialToken::Shr), // >>
                '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::Geq))),
                _ => (1, SpecialToken::RAngle),
            },
            '*' => match self.peek_char() {
                '*' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Pure(
                    PureBinaryOpr::Power,
                ))),
                '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                    PureBinaryOpr::Mul,
                )))),
                _ => (
                    1,
                    SpecialToken::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::Mul)),
                ),
            },
            '/' => match self.peek_char() {
                '/' => return None,
                '>' => self.pass_two(SpecialToken::XmlKet),
                '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                    PureBinaryOpr::Div,
                )))),
                _ => (
                    1,
                    SpecialToken::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::Div)),
                ),
            },
            '+' => match self.peek_char() {
                '+' => self.pass_two(SpecialToken::Incr),
                '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                    PureBinaryOpr::Add,
                )))),
                _ => (
                    1,
                    SpecialToken::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::Add)),
                ),
            },
            '!' => match self.peek_char() {
                '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::Neq))),
                '!' => self.pass_two(SpecialToken::DoubleExclamation),
                _ => (1, SpecialToken::Exclamation),
            },
            '?' => (1, SpecialToken::QuestionMark),
            c => return Some((1, TokenKind::Unrecognized(c).into())),
        };
        Some((len, RawTokenKind::Certain(TokenKind::Special(special))))
    }
}

impl<'token_line, 'lex: 'token_line> Iterator for RawTokenIter<'token_line, 'lex> {
    type Item = RawToken;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((j, c)) = self.char_iter.next() {
            if c == ' ' {
                self.skip_whitespaces();
                return self.next();
            } else if c.is_alphabetic() || c == '_' {
                self.buffer.push(c);
                Some(self.next_word(j).into())
            } else if c.is_digit(10) {
                self.buffer.push(c);
                Some(self.next_number(j).into())
            } else {
                self.next_special(j, c)
            }
        } else {
            None
        }
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
