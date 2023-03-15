use super::*;
use husky_opn_syntax::*;

use husky_text::{TextCharIter, TextRange};

use husky_word::{is_valid_ident_first_char, WordDb};
use std::str::FromStr;

pub(crate) struct RangedPretoken {
    pub(crate) range: TextRange,
    pub(crate) token: Pretoken,
}

impl RangedPretoken {
    fn new(i: u32, start: u32, end: u32, token: Pretoken) -> Self {
        RangedPretoken {
            range: husky_text::new_same_line(i, start, end),
            token,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Pretoken {
    Certain(Token),
    Literal(Literal),
    NewLine,
    Ambiguous(AmbiguousPretoken),
    Comment,
    Err(TokenError),
}

impl From<AmbiguousPretoken> for Pretoken {
    fn from(v: AmbiguousPretoken) -> Self {
        Self::Ambiguous(v)
    }
}

impl From<IntegerLikeLiteral> for Pretoken {
    fn from(val: IntegerLikeLiteral) -> Self {
        Pretoken::Certain(Token::Literal(Literal::Integer(val)))
    }
}

impl From<FloatLiteral> for Pretoken {
    fn from(val: FloatLiteral) -> Self {
        Pretoken::Certain(Token::Literal(Literal::Float(val)))
    }
}

impl const From<EndKeyword> for Pretoken {
    fn from(kw: EndKeyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AmbiguousPretoken {
    SubOrMinus,
    For,
}

impl AmbiguousPretoken {
    pub fn code(self) -> &'static str {
        match self {
            AmbiguousPretoken::SubOrMinus => todo!(),
            AmbiguousPretoken::For => "for",
        }
    }
}

impl From<Token> for Pretoken {
    fn from(kind: Token) -> Self {
        Pretoken::Certain(kind)
    }
}

impl From<Punctuation> for Pretoken {
    fn from(value: Punctuation) -> Self {
        Pretoken::Certain(value.into())
    }
}

impl From<Keyword> for Pretoken {
    fn from(kw: Keyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<StmtKeyword> for Pretoken {
    fn from(kw: StmtKeyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<TypeEntityKeyword> for Pretoken {
    fn from(kw: TypeEntityKeyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<PatternKeyword> for Pretoken {
    fn from(kw: PatternKeyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<ConfigKeyword> for Pretoken {
    fn from(kw: ConfigKeyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<WordOpr> for Pretoken {
    fn from(kw: WordOpr) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<FormKeyword> for Pretoken {
    fn from(val: FormKeyword) -> Self {
        Pretoken::Certain(val.into())
    }
}

impl From<PronounKeyword> for Pretoken {
    fn from(val: PronounKeyword) -> Self {
        Pretoken::Certain(val.into())
    }
}

impl From<BoolLiteral> for Pretoken {
    fn from(value: BoolLiteral) -> Self {
        Pretoken::Certain(value.into())
    }
}

pub(crate) struct PretokenStream<'a, 'b> {
    db: &'a dyn TokenDb,
    buffer: String,
    char_iter: TextCharIter<'b>,
}

impl<'a, 'b> PretokenStream<'a, 'b> {
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

impl<'a, 'b: 'a> PretokenStream<'a, 'b> {
    fn next_token_variant(&mut self) -> Option<Pretoken> {
        let c = self.char_iter.next()?;
        assert_ne!(c, ' ');
        match c {
            '\n' => Some(Pretoken::NewLine),
            '\'' => Some(self.next_char_or_lifetime_or_label()),
            '"' => {
                match self.next_string_literal(StringLiteralKind::SingleLine) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        // skip this line
                        while let Some(c) = self.char_iter.next() {
                            if c == '\n' {
                                break;
                            }
                        }
                        Some(Pretoken::Err(e))
                    }
                }
            }
            c if c.is_alphabetic() || c == '_' => {
                self.buffer.push(c);
                Some(self.next_word())
            }
            c if c.is_digit(10) => {
                self.buffer.push(c);
                Some(self.next_number())
            }
            c if c == '/' && self.char_iter.peek() == Some('/') => {
                while let Some(c) = self.char_iter.peek() {
                    if c == '\n' {
                        break;
                    } else {
                        self.char_iter.next();
                    }
                }
                Some(Pretoken::Comment)
            }
            c => self.next_punctuation(c),
        }
    }

    /// assume a previous single quote has been taken
    fn next_char_or_lifetime_or_label(&mut self) -> Pretoken {
        let Some((fst, snd)) = self.char_iter.peek_two()
            else {
                return Pretoken::Err(TokenError::NothingAfterSingleQuote)
            };
        match fst {
            '\\' => todo!(),
            fst if is_valid_ident_first_char(fst) => match snd {
                Some('\'') => {
                    self.char_iter.next();
                    self.char_iter.next();
                    Pretoken::Literal(Literal::Char(CharLiteral::Basic(fst)))
                }
                _ => self.next_auxiliary_identifier(),
            },
            _ => {
                self.char_iter.next();
                Pretoken::Err(TokenError::InvalidLabel)
            }
        }
    }

    fn next_auxiliary_identifier(&mut self) -> Pretoken {
        while let Some(c) = self.char_iter.peek() {
            if is_word_char(c) {
                self.eat_char();
            } else {
                break;
            }
        }
        assert!(self.buffer.len() > 0);
        let word = &self.buffer;
        let pretoken = match self.db.it_auxiliary_ident_borrowed(word) {
            Some(identifier) => Token::Label(identifier).into(),
            None => Pretoken::Err(TokenError::InvalidIdent),
        };
        self.buffer.clear();
        pretoken
    }

    fn skip_whitespaces(&mut self) {
        while let Some(' ') = self.char_iter.peek() {
            self.char_iter.next();
        }
    }

    fn next_word(&mut self) -> Pretoken {
        while let Some(c) = self.char_iter.peek() {
            if is_word_char(c) {
                self.eat_char();
            } else {
                break;
            }
        }
        assert!(self.buffer.len() > 0);
        let word = &self.buffer;
        let pretoken = if let Some(pretoken) = new_reserved_word(self.db, word) {
            pretoken
        } else {
            match self.db.it_ident_borrowed(word) {
                Some(identifier) => Token::Ident(identifier).into(),
                None => Pretoken::Err(TokenError::InvalidIdent),
            }
        };
        self.buffer.clear();
        pretoken
    }

    fn next_number(&mut self) -> Pretoken {
        let radix = 10;
        self.eat_chars_with(|c| char::is_digit(c, radix));
        if self.try_eat_char(|c| c == '.').is_some() {
            // parse float type
            self.eat_chars_with(|c| c.is_digit(radix));
            let float_suffix = self.get_str_slice_with(|c| c.is_alphanumeric());
            self.buffer.clear();
            match float_suffix {
                "" => FloatLiteral::Unspecified.into(),
                "f8" => todo!(),
                "f16" => todo!(),
                "f32" => todo!(),
                "f64" => todo!(),
                "f128" => todo!(),
                "f256" => todo!(),
                _ => Pretoken::Err(TokenError::InvalidFloatSuffix),
            }
        } else {
            let integer_suffix = self.get_str_slice_with(|c| c.is_alphanumeric());
            let token: Pretoken = match integer_suffix {
                "" => IntegerLikeLiteral::Unspecified.into(),
                "i8" => todo!(),
                "i16" => todo!(),
                "i32" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenError::ParseIntError)
                    };
                    IntegerLikeLiteral::I32(i).into()
                }
                "i64" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenError::ParseIntError)
                    };
                    IntegerLikeLiteral::I64(i).into()
                }
                "i128" => todo!(),
                "i256" => todo!(),
                "r8" => todo!(),
                "r16" => todo!(),
                "r32" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenError::ParseIntError)
                    };
                    IntegerLikeLiteral::R32(i).into()
                }
                "r64" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenError::ParseIntError)
                    };
                    IntegerLikeLiteral::R64(i).into()
                }
                "r128" => todo!(),
                "r256" => todo!(),
                "u8" => todo!(),
                "u16" => todo!(),
                "u32" => todo!(),
                "u64" => todo!(),
                "u128" => todo!(),
                "u256" => todo!(),
                _invalid_integer_suffix => return Pretoken::Err(TokenError::InvalidIntegerSuffix),
            };
            self.buffer.clear();
            token
        }
    }

    fn take_buffer<T>(&mut self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        std::mem::take(&mut self.buffer).parse::<T>().unwrap()
    }

    fn peek_char(&mut self) -> Option<char> {
        self.char_iter.peek()
    }

    fn pass_two(&mut self, special: Punctuation) -> Punctuation {
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

    fn next_punctuation(&mut self, c_start: char) -> Option<Pretoken> {
        Some(
            match c_start {
                '=' => match self.peek_char() {
                    Some('=') => self.pass_two(Punctuation::Binary(BinaryOpr::Comparison(
                        BinaryComparisonOpr::Eq,
                    ))),
                    _ => Punctuation::Eq,
                },
                ':' => match self.peek_char() {
                    Some('=') => self.pass_two(Punctuation::DeriveAssign),
                    Some(':') => {
                        self.char_iter.next();
                        match self.peek_char() {
                            Some('<') => {
                                self.char_iter.next();
                                Punctuation::ColonColonLAngle
                            }
                            _ => Punctuation::ColonColon,
                        }
                    }
                    _ => Punctuation::Colon,
                },
                '(' => Punctuation::Bra(Bracket::Par),
                '[' => Punctuation::Bra(Bracket::Box),
                '{' => Punctuation::Bra(Bracket::Curl),
                ')' => Punctuation::Ket(Bracket::Par),
                ']' => Punctuation::Ket(Bracket::Box),
                '}' => Punctuation::Ket(Bracket::Curl),
                ',' => Punctuation::Comma,
                '@' => match self.peek_char() {
                    Some('=') => self.pass_two(Punctuation::AtEq),
                    _ => Punctuation::At,
                },
                '&' => match self.peek_char() {
                    Some('&') => self.pass_two(Punctuation::Binary(BinaryOpr::ShortCircuitLogic(
                        BinaryShortcuitLogicOpr::And,
                    ))),
                    Some('=') => self.pass_two(Punctuation::Binary(BinaryOpr::AssignClosed(
                        BinaryClosedOpr::BitAnd,
                    ))),
                    _ => Punctuation::Ambersand,
                },
                '|' => match self.peek_char() {
                    Some('|') => self.pass_two(Punctuation::DoubleVertical),
                    Some('=') => self.pass_two(Punctuation::Binary(BinaryOpr::AssignClosed(
                        BinaryClosedOpr::BitOr,
                    ))),
                    _ => Punctuation::Vertical,
                },
                '~' => Punctuation::Tilde,
                '.' => match self.peek_char() {
                    Some('.') => self.pass_two(Punctuation::DotDot),
                    _ => Punctuation::Dot,
                },
                ';' => Punctuation::Semicolon,
                '%' => Punctuation::Binary(BinaryOpr::Closed(BinaryClosedOpr::RemEuclid)),

                '-' => match self.peek_char() {
                    Some('=') => self.pass_two(Punctuation::Binary(BinaryOpr::AssignClosed(
                        BinaryClosedOpr::Sub,
                    ))),
                    Some('-') => self.pass_two(Punctuation::Suffix(SuffixOpr::Decr)),
                    Some('>') => self.pass_two(Punctuation::Binary(BinaryOpr::Curry)),
                    _ => return Some(Pretoken::Ambiguous(AmbiguousPretoken::SubOrMinus)),
                },
                '<' => match self.peek_char() {
                    Some('<') => {
                        self.pass_two(Punctuation::Binary(BinaryOpr::Shift(BinaryShiftOpr::Shl)))
                    }
                    Some('=') => self.pass_two(Punctuation::Binary(BinaryOpr::Comparison(
                        BinaryComparisonOpr::Leq,
                    ))),
                    _ => Punctuation::LaOrLt,
                },
                '>' => match self.peek_char() {
                    Some('>') => self.pass_two(Punctuation::Shr), // >>
                    Some('=') => self.pass_two(Punctuation::Binary(BinaryOpr::Comparison(
                        BinaryComparisonOpr::Geq,
                    ))),
                    _ => Punctuation::RaOrGt,
                },
                '$' => Punctuation::Sheba,
                '*' => match self.peek_char() {
                    Some('*') => self.pass_two(Punctuation::Binary(BinaryOpr::Closed(
                        BinaryClosedOpr::Power,
                    ))),
                    Some('=') => self.pass_two(Punctuation::Binary(BinaryOpr::AssignClosed(
                        BinaryClosedOpr::Mul,
                    ))),
                    _ => Punctuation::Star,
                },
                '/' => match self.peek_char() {
                    Some('/') => unreachable!(),
                    Some('>') => self.pass_two(Punctuation::XmlKet),
                    Some('=') => self.pass_two(Punctuation::Binary(BinaryOpr::AssignClosed(
                        BinaryClosedOpr::Div,
                    ))),
                    _ => Punctuation::Binary(BinaryOpr::Closed(BinaryClosedOpr::Div)),
                },
                '+' => match self.peek_char() {
                    Some('+') => self.pass_two(Punctuation::Suffix(SuffixOpr::Incr)),
                    Some('=') => self.pass_two(Punctuation::Binary(BinaryOpr::AssignClosed(
                        BinaryClosedOpr::Add,
                    ))),
                    _ => Punctuation::Binary(BinaryOpr::Closed(BinaryClosedOpr::Add)),
                },
                '!' => match self.peek_char() {
                    Some('=') => self.pass_two(Punctuation::Binary(BinaryOpr::Comparison(
                        BinaryComparisonOpr::Neq,
                    ))),
                    Some('!') => self.pass_two(Punctuation::DoubleExclamation),
                    _ => Punctuation::Exclamation,
                },
                '?' => Punctuation::Question,
                '#' => Punctuation::PoundSign,
                '∀' => Punctuation::ForAll,
                '∃' => Punctuation::Exists,
                c => return Some(Pretoken::Err(TokenError::UnrecognizedChar(c))),
            }
            .into(),
        )
    }

    fn next_string_literal(
        &mut self,
        string_literal_kind: StringLiteralKind,
    ) -> TokenResult<Pretoken> {
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
                            c => return Err(TokenError::UnexpectedCharAfterBackslash(c)),
                        }
                    } else {
                        return Err(TokenError::IncompleteStringLiteralBeforeEof);
                    }
                }
                '\n' => match string_literal_kind {
                    StringLiteralKind::SingleLine => {
                        return Err(TokenError::IncompleteStringLiteralBeforeEol);
                    }
                    StringLiteralKind::MultipleLine => s.push('\n'),
                },
                c => s.push(c),
            }
        }
        Ok(Pretoken::Literal(Literal::String(StringLiteral::new(
            self.db, s,
        ))))
    }
}

enum StringLiteralKind {
    SingleLine,
    MultipleLine,
}

impl<'token_line, 'lex: 'token_line> Iterator for PretokenStream<'token_line, 'lex> {
    type Item = RangedPretoken;

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
                Some(RangedPretoken {
                    range: (start..self.char_iter.current_position()).into(),
                    token: variant,
                })
            }
        }
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
