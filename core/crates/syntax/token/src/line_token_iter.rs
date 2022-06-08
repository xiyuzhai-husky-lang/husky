use print_utils::p;
use text::CharIter;
use word::WordAllocator;

use crate::*;

pub(crate) struct LineTokenIter<'token_line, 'lex: 'token_line> {
    word_unique_allocator: &'lex WordAllocator,
    line_index: usize,
    buffer: String,
    char_iter: CharIter<'token_line>,
}

impl<'token_line, 'lex: 'token_line> LineTokenIter<'token_line, 'lex> {
    pub fn new(
        word_unique_allocator: &'lex WordAllocator,
        line_index: usize,
        mut char_iter: CharIter<'token_line>,
    ) -> (TextIndent, Self) {
        let mut buffer = String::new();
        buffer.reserve(100);
        let indent = TextIndent::from(&mut char_iter);
        (
            indent,
            Self {
                word_unique_allocator,
                line_index,
                buffer,
                char_iter,
            },
        )
    }
}

impl<'token_line, 'lex: 'token_line> LineTokenIter<'token_line, 'lex> {
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

    fn next_number(&mut self, j_start: usize) -> Token {
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
                Token::new(
                    self.line_index,
                    j_start,
                    j_start + len,
                    TokenKind::PrimitiveLiteral(self.take_buffer_f32().into()),
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
                                TokenKind::IllFormedLiteral(self.take_buffer_b64().into()),
                            )
                        } else {
                            self.ignore_char();
                            if is_word_char(self.peek_char()) {
                                todo!()
                            } else {
                                (
                                    self.buffer.len() + 3,
                                    TokenKind::PrimitiveLiteral(self.take_buffer_b32().into()),
                                )
                            }
                        }
                    }
                    '6' => {
                        self.ignore_char();
                        if self.peek_char() != '4' {
                            todo!()
                        }
                        todo!()
                    }
                    _ => (
                        self.buffer.len() + 1,
                        TokenKind::IllFormedLiteral(self.take_buffer_b64().into()),
                    ),
                };
                Token::new(self.line_index, j_start, j_start + token_len, kind)
            }
            'i' => {
                // i64
                todo!()
            }
            default => {
                if default.is_alphabetic() {
                    // letter other than 'b' or 'i' after integer literal is not allowed
                    let mut token_len = self.buffer.len() + 1;
                    while self.peek_char().is_alphabetic() {
                        self.ignore_char();
                        token_len += 1;
                    }
                    Token::new(
                        self.line_index,
                        j_start,
                        j_start + token_len,
                        TokenKind::IllFormedLiteral(self.take_buffer_b64().into()),
                    )
                } else {
                    // i32
                    let len = self.buffer.len();
                    Token::new(
                        self.line_index,
                        j_start,
                        j_start + len,
                        TokenKind::PrimitiveLiteral(self.take_buffer_i32().into()),
                    )
                }
            }
        }
    }

    fn take_buffer_word(&mut self) -> word::WordPtr {
        let word = self
            .word_unique_allocator
            .alloc(std::mem::take(&mut self.buffer));
        self.buffer.clear();
        word
    }

    fn take_buffer_i32(&mut self) -> i32 {
        std::mem::take(&mut self.buffer).parse::<i32>().unwrap()
    }

    fn take_buffer_f32(&mut self) -> f32 {
        std::mem::take(&mut self.buffer).parse::<f32>().unwrap()
    }

    fn take_buffer_b32(&mut self) -> u32 {
        std::mem::take(&mut self.buffer).parse::<u32>().unwrap()
    }

    fn take_buffer_b64(&mut self) -> u64 {
        std::mem::take(&mut self.buffer).parse::<u64>().unwrap()
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

    fn next_special(&mut self, j_start: usize, c_start: char) -> Option<Token> {
        let (len, special) = match c_start {
            '=' => match self.peek_char() {
                '=' => self.pass_two(SpecialToken::Eq),
                '>' => self.pass_two(SpecialToken::HeavyArrow),
                _ => (1, SpecialToken::Assign),
            },
            ':' => match self.peek_char() {
                '=' => self.pass_two(SpecialToken::DeriveAssign),
                ':' => self.pass_two(SpecialToken::DoubleColon),
                _ => (1, SpecialToken::Colon),
            },
            '(' => (1, SpecialToken::LPar),
            '[' => (1, SpecialToken::LBox),
            '{' => (1, SpecialToken::LCurl),
            ')' => (1, SpecialToken::RPar),
            ']' => (1, SpecialToken::RBox),
            '}' => (1, SpecialToken::RCurl),
            ',' => (1, SpecialToken::Comma),
            '@' => (1, SpecialToken::At),
            '&' => match self.peek_char() {
                '&' => self.pass_two(SpecialToken::And),
                '=' => self.pass_two(SpecialToken::BitAndAssign),
                _ => (1, SpecialToken::Ambersand),
            },
            '|' => match self.peek_char() {
                '|' => self.pass_two(SpecialToken::DoubleVertical),
                '=' => self.pass_two(SpecialToken::BitOrAssign),
                _ => (1, SpecialToken::Vertical),
            },
            '~' => (1, SpecialToken::BitNot),
            '.' => (1, SpecialToken::MemberAccess),
            ';' => (1, SpecialToken::Semicolon),
            '%' => (1, SpecialToken::Modulo),
            '-' => match self.peek_char() {
                '=' => self.pass_two(SpecialToken::SubAssign),
                '-' => self.pass_two(SpecialToken::Decr),
                '>' => self.pass_two(SpecialToken::LightArrow),
                _ => (1, SpecialToken::SubOrMinus),
            },
            '<' => match self.peek_char() {
                '<' => self.pass_two(SpecialToken::Shl), // <<
                '=' => self.pass_two(SpecialToken::Leq),
                _ => (1, SpecialToken::LAngle),
            },
            '>' => match self.peek_char() {
                '>' => self.pass_two(SpecialToken::Shr), // >>
                '=' => self.pass_two(SpecialToken::Geq),
                _ => (1, SpecialToken::RAngle),
            },
            '*' => match self.peek_char() {
                '*' => self.pass_two(SpecialToken::Power),
                '=' => self.pass_two(SpecialToken::MulAssign),
                _ => (1, SpecialToken::Star),
            },
            '/' => match self.peek_char() {
                '/' => return None,
                '>' => self.pass_two(SpecialToken::XmlKet),
                '=' => self.pass_two(SpecialToken::DivAssign),
                _ => (1, SpecialToken::Div),
            },
            '+' => match self.peek_char() {
                '+' => self.pass_two(SpecialToken::Incr),
                '=' => self.pass_two(SpecialToken::AddAssign),
                _ => (1, SpecialToken::Add),
            },
            '!' => match self.peek_char() {
                '=' => self.pass_two(SpecialToken::Neq),
                '!' => self.pass_two(SpecialToken::DoubleExclamation),
                _ => (1, SpecialToken::Exclamation),
            },
            c => {
                return Some(Token::new(
                    self.line_index,
                    j_start,
                    j_start + 1,
                    TokenKind::Unrecognized(c),
                ))
            }
        };
        Some(Token::new(
            self.line_index,
            j_start,
            j_start + len,
            TokenKind::Special(special),
        ))
    }
}

impl<'token_line, 'lex: 'token_line> Iterator for LineTokenIter<'token_line, 'lex> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((j, c)) = self.char_iter.next() {
            if c == ' ' {
                self.skip_whitespaces();
                return self.next();
            } else if c.is_alphabetic() || c == '_' {
                self.buffer.push(c);
                Some(self.next_word(j))
            } else if c.is_digit(10) {
                self.buffer.push(c);
                Some(self.next_number(j))
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
