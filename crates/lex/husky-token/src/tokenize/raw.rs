use super::*;
use husky_opn_syntax::*;
use husky_text::{TextCharIter, TextRange};
use husky_word::WordDb;
use std::str::FromStr;

pub(crate) struct RawToken {
    pub(crate) range: TextRange,
    pub(crate) variant: RawTokenVariant,
}

impl RawToken {
    fn new(i: u32, start: u32, end: u32, variant: RawTokenVariant) -> Self {
        RawToken {
            range: husky_text::new_same_line(i, start, end),
            variant,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum RawTokenVariant {
    Certain(TokenKind),
    Literal(LiteralToken),
    SubOrMinus,
    NewLine,
    Special(AmbiguousSpecial),
    Comment,
    Err(TokenError),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AmbiguousSpecial {
    For,
}

impl From<TokenKind> for RawTokenVariant {
    fn from(kind: TokenKind) -> Self {
        RawTokenVariant::Certain(kind)
    }
}

impl From<SpecialToken> for RawTokenVariant {
    fn from(value: SpecialToken) -> Self {
        RawTokenVariant::Certain(value.into())
    }
}

impl From<Keyword> for RawTokenVariant {
    fn from(kw: Keyword) -> Self {
        RawTokenVariant::Certain(kw.into())
    }
}

impl From<StmtKeyword> for RawTokenVariant {
    fn from(kw: StmtKeyword) -> Self {
        RawTokenVariant::Certain(kw.into())
    }
}

impl From<TypeKeyword> for RawTokenVariant {
    fn from(kw: TypeKeyword) -> Self {
        RawTokenVariant::Certain(kw.into())
    }
}

impl From<LiasonKeyword> for RawTokenVariant {
    fn from(kw: LiasonKeyword) -> Self {
        RawTokenVariant::Certain(kw.into())
    }
}

impl From<ConfigKeyword> for RawTokenVariant {
    fn from(kw: ConfigKeyword) -> Self {
        RawTokenVariant::Certain(kw.into())
    }
}

impl From<AttrKeyword> for RawTokenVariant {
    fn from(kw: AttrKeyword) -> Self {
        RawTokenVariant::Certain(kw.into())
    }
}

impl From<WordOpr> for RawTokenVariant {
    fn from(kw: WordOpr) -> Self {
        RawTokenVariant::Certain(kw.into())
    }
}

impl Into<RawTokenVariant> for FormKeyword {
    fn into(self) -> RawTokenVariant {
        RawTokenVariant::Certain(self.into())
    }
}

impl From<Token> for RawToken {
    fn from(value: Token) -> Self {
        Self {
            range: value.range,
            variant: RawTokenVariant::Certain(value.kind),
        }
    }
}

pub(crate) struct RawTokenIter<'a, 'b> {
    db: &'a dyn TokenDb,
    buffer: String,
    char_iter: TextCharIter<'b>,
}

impl<'a, 'b> RawTokenIter<'a, 'b> {
    pub fn new(db: &'a dyn TokenDb, char_iter: TextCharIter<'b>) -> Self {
        let mut buffer = String::new();
        buffer.reserve_exact(100);
        Self {
            db,
            buffer,
            char_iter,
        }
    }
}

impl<'a, 'b: 'a> RawTokenIter<'a, 'b> {
    fn skip_whitespaces(&mut self) {
        while let Some(' ') = self.char_iter.peek() {
            self.char_iter.next();
        }
    }

    fn next_word(&mut self) -> RawTokenVariant {
        while let Some(c) = self.char_iter.peek() {
            if is_word_char(c) {
                self.eat_char();
            } else {
                break;
            }
        }
        let _len = self.buffer.len();
        self.take_buffer_word()
    }

    fn next_number(&mut self) -> RawTokenVariant {
        let radix = 10;
        self.eat_chars_with(|c| char::is_digit(c, radix));
        if self.try_eat_char(|c| c == '.').is_some() {
            // parse float type
            self.eat_chars_with(|c| c.is_digit(radix));
            let float_suffix = self.get_str_slice_with(|c| c.is_alphanumeric());
            match float_suffix {
                "f8" => todo!(),
                "f16" => todo!(),
                "f32" => todo!(),
                "f64" => todo!(),
                "f128" => todo!(),
                "f256" => todo!(),
                invalid_float_suffix => todo!(),
            }
        } else {
            let integer_suffix = self.get_str_slice_with(|c| c.is_alphanumeric());
            match integer_suffix {
                "i8" => todo!(),
                "i16" => todo!(),
                "i32" => todo!(),
                "i64" => todo!(),
                "i128" => todo!(),
                "i256" => todo!(),
                "r8" => todo!(),
                "r16" => todo!(),
                "r32" => todo!(),
                "r64" => todo!(),
                "r128" => todo!(),
                "r256" => todo!(),
                "u8" => todo!(),
                "u16" => todo!(),
                "u32" => todo!(),
                "u64" => todo!(),
                "u128" => todo!(),
                "u256" => todo!(),
                invalid_integer_suffix => todo!(),
            }
        }
    }

    fn take_buffer_word(&mut self) -> RawTokenVariant {
        let word = std::mem::take(&mut self.buffer);
        self.new_word(word)
    }

    fn new_word(&self, word: String) -> RawTokenVariant {
        if let Some(token_kind) = new_reserved_word(self.db, &word) {
            // ad hoc
            token_kind
        } else {
            TokenKind::Identifier(self.db.it_ident_owned(word).expect("todo")).into()
        }
    }

    fn take_buffer<T>(&mut self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        std::mem::take(&mut self.buffer).parse::<T>().unwrap()
    }

    fn peek_char(&mut self) -> char {
        if let Some(c) = self.char_iter.peek() {
            c
        } else {
            0.into()
        }
    }

    fn pass_two(&mut self, special: SpecialToken) -> SpecialToken {
        self.char_iter.next();
        special
    }

    fn eat_char(&mut self) {
        let c = self.char_iter.next().expect("what");
        self.buffer.push(c);
    }

    fn try_eat_char(&mut self, predicate: impl FnOnce(char) -> bool) -> Option<char> {
        if let Some(c) = self.char_iter.peek() {
            if predicate(c) {
                self.eat_char();
                Some(c)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn eat_chars_with(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(c) = self.char_iter.peek() {
            if predicate(c) {
                self.eat_char();
            } else {
                break;
            }
        }
    }

    fn get_str_slice_with(&mut self, predicate: impl Fn(char) -> bool) -> &'b str {
        self.char_iter.get_str_slice_with(predicate)
    }

    fn ignore_char(&mut self) {
        let _c = self.char_iter.next().expect("what");
    }

    fn next_special(&mut self, c_start: char) -> Option<RawTokenVariant> {
        Some(
            match c_start {
                '=' => match self.peek_char() {
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Comparison(
                        BinaryComparisonOpr::Eq,
                    ))),
                    _ => SpecialToken::BinaryOpr(BinaryOpr::Assign(None)),
                },
                ':' => match self.peek_char() {
                    '=' => self.pass_two(SpecialToken::DeriveAssign),
                    ':' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::ScopeResolution)),
                    _ => SpecialToken::Colon,
                },
                '(' => SpecialToken::Bra(Bracket::Par),
                '[' => SpecialToken::Bra(Bracket::Box),
                '{' => SpecialToken::Bra(Bracket::Curl),
                ')' => SpecialToken::Ket(Bracket::Par),
                ']' => SpecialToken::Ket(Bracket::Box),
                '}' => SpecialToken::Ket(Bracket::Curl),
                ',' => SpecialToken::Comma,
                '@' => SpecialToken::At,
                '&' => match self.peek_char() {
                    '&' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::ShortcuitLogic(
                        BinaryShortcuitLogicOpr::And,
                    ))),
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                        BinaryPureClosedOpr::BitAnd,
                    )))),
                    _ => SpecialToken::Ambersand,
                },
                '|' => match self.peek_char() {
                    '|' => self.pass_two(SpecialToken::DoubleVertical),
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                        BinaryPureClosedOpr::BitOr,
                    )))),
                    _ => SpecialToken::Vertical,
                },
                '~' => SpecialToken::BitNot,
                '.' => SpecialToken::Dot,
                ';' => SpecialToken::Semicolon,
                '%' => {
                    SpecialToken::BinaryOpr(BinaryOpr::PureClosed(BinaryPureClosedOpr::RemEuclid))
                }

                '-' => match self.peek_char() {
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                        BinaryPureClosedOpr::Sub,
                    )))),
                    '-' => self.pass_two(SpecialToken::Decr),
                    '>' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Curry)),
                    _ => return Some(RawTokenVariant::SubOrMinus),
                },
                '<' => match self.peek_char() {
                    '<' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::PureClosed(
                        BinaryPureClosedOpr::Shl,
                    ))),
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Comparison(
                        BinaryComparisonOpr::Leq,
                    ))),
                    _ => SpecialToken::LAngle,
                },
                '>' => match self.peek_char() {
                    // '>' => self.pass_two(SpecialToken::Shr), // >>
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Comparison(
                        BinaryComparisonOpr::Geq,
                    ))),
                    _ => SpecialToken::RAngle,
                },
                '*' => match self.peek_char() {
                    '*' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::PureClosed(
                        BinaryPureClosedOpr::Power,
                    ))),
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                        BinaryPureClosedOpr::Mul,
                    )))),
                    _ => SpecialToken::BinaryOpr(BinaryOpr::PureClosed(BinaryPureClosedOpr::Mul)),
                },
                '/' => match self.peek_char() {
                    '/' => unreachable!(),
                    '>' => self.pass_two(SpecialToken::XmlKet),
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                        BinaryPureClosedOpr::Div,
                    )))),
                    _ => SpecialToken::BinaryOpr(BinaryOpr::PureClosed(BinaryPureClosedOpr::Div)),
                },
                '+' => match self.peek_char() {
                    '+' => self.pass_two(SpecialToken::Incr),
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                        BinaryPureClosedOpr::Add,
                    )))),
                    _ => SpecialToken::BinaryOpr(BinaryOpr::PureClosed(BinaryPureClosedOpr::Add)),
                },
                '!' => match self.peek_char() {
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Comparison(
                        BinaryComparisonOpr::Neq,
                    ))),
                    '!' => self.pass_two(SpecialToken::DoubleExclamation),
                    _ => SpecialToken::Exclamation,
                },
                '?' => SpecialToken::QuestionMark,
                '#' => SpecialToken::PoundSign,
                c => return Some(RawTokenVariant::Err(TokenError::UnrecognizedChar(c))),
            }
            .into(),
        )
    }

    fn next_string_literal(&mut self) -> TokenResult<RawTokenVariant> {
        let mut s = String::new();
        while let Some(c) = self.char_iter.next() {
            match c {
                '"' => break,
                '\\' => {
                    if let Some(c) = self.char_iter.next() {
                        match c {
                            '"' => s.push('"'),
                            '\\' => s.push('\\'),
                            'n' => s.push('\n'),
                            'r' => s.push('\r'),
                            't' => s.push('\t'),
                            c => return Err(TokenError::UnexpectedCharAfterBackslash),
                        }
                    } else {
                        return Err(TokenError::IncompleteStringLiteral);
                    }
                }
                '\n' => todo!(),
                c => s.push(c),
            }
        }
        Ok(RawTokenVariant::Literal(LiteralToken::String(
            StringLiteral::new(self.db, s),
        )))
    }

    fn next_token_variant(&mut self) -> Option<RawTokenVariant> {
        let c = self.char_iter.next()?;
        if c == '\n' {
            Some(RawTokenVariant::NewLine)
        } else if c == '"' {
            match self.next_string_literal() {
                Ok(v) => Some(v),
                Err(e) => {
                    // skip this line
                    while let Some(c) = self.char_iter.next() {
                        if c == '\n' {
                            break;
                        }
                    }
                    Some(RawTokenVariant::Err(e))
                }
            }
        } else if c == ' ' {
            unreachable!()
        } else if c.is_alphabetic() || c == '_' {
            self.buffer.push(c);
            Some(self.next_word())
        } else if c.is_digit(10) {
            self.buffer.push(c);
            Some(self.next_number())
        } else if c == '/' && self.char_iter.peek() == Some('/') {
            while let Some(c) = self.char_iter.peek() {
                if c == '\n' {
                    break;
                } else {
                    self.char_iter.next();
                }
            }
            Some(RawTokenVariant::Comment)
        } else {
            self.next_special(c)
        }
    }
}

impl<'token_line, 'lex: 'token_line> Iterator for RawTokenIter<'token_line, 'lex> {
    type Item = RawToken;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.char_iter.peek()?;
        match c {
            ' ' => {
                self.skip_whitespaces();
                self.next()
            }
            _ => {
                let start = self.char_iter.current_position();
                let variant = self.next_token_variant()?;
                Some(RawToken {
                    range: (start..self.char_iter.current_position()).into(),
                    variant,
                })
            }
        }
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
