use text::CharIter;
use word::WordInterner;

use crate::*;

pub(crate) struct LineTokenIter<'token_line, 'lex: 'token_line> {
    db: &'lex WordInterner,
    line_index: usize,
    buffer: String,
    char_iter: CharIter<'token_line>,
}

impl<'token_line, 'lex: 'token_line> LineTokenIter<'token_line, 'lex> {
    pub fn new(
        word_interner: &'lex WordInterner,
        line_index: usize,
        mut char_iter: CharIter<'token_line>,
    ) -> (Indent, Self) {
        let mut buffer = String::new();
        buffer.reserve(100);
        let indent = Indent::from(&mut char_iter);
        (
            indent,
            Self {
                db: word_interner,
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
                self.eat();
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

        fn is_word_char(c: char) -> bool {
            c.is_alphanumeric() || c == '_'
        }
    }

    fn next_number(&mut self, j_start: usize) -> Token {
        while self.peek().is_digit(10) {
            self.eat()
        }
        if self.peek() == '.' {
            self.eat();
            while self.peek().is_digit(10) {
                self.eat()
            }
            let len = self.buffer.len();
            Token::new(
                self.line_index,
                j_start,
                j_start + len,
                TokenKind::F32Literal(self.take_buffer_f32()),
            )
        } else {
            let len = self.buffer.len();
            Token::new(
                self.line_index,
                j_start,
                j_start + len,
                TokenKind::I32Literal(self.take_buffer_i32()),
            )
        }
    }

    fn take_buffer_word(&mut self) -> word::Word {
        let word = self.db.id_by_ref(&self.buffer);
        self.buffer.clear();
        word
    }

    fn take_buffer_i32(&mut self) -> i32 {
        std::mem::take(&mut self.buffer).parse::<i32>().unwrap()
    }

    fn take_buffer_f32(&mut self) -> f32 {
        std::mem::take(&mut self.buffer).parse::<f32>().unwrap()
    }

    fn peek(&mut self) -> char {
        if let Some((_, c)) = self.char_iter.peek() {
            *c
        } else {
            0.into()
        }
    }

    fn pass(&mut self, special: Special) -> (usize, Special) {
        self.char_iter.next();
        (2, special)
    }

    fn eat(&mut self) {
        let (_, c) = self.char_iter.next().expect("what");
        self.buffer.push(c);
    }

    fn next_special(&mut self, j_start: usize, c_start: char) -> Option<Token> {
        let (len, special) = match c_start {
            '=' => match self.peek() {
                '=' => self.pass(Special::Eq),
                '>' => self.pass(Special::HeavyArrow),
                _ => (1, Special::Assign),
            },
            ':' => match self.peek() {
                ':' => self.pass(Special::DoubleColon),
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
                '&' => self.pass(Special::And),
                _ => (1, Special::Ambersand),
            },
            '|' => match self.peek() {
                '|' => self.pass(Special::DoubleVertical),
                _ => (1, Special::Vertical),
            },
            '~' => (1, Special::BitNot),
            '.' => (1, Special::MemberAccess),
            '%' => (1, Special::Modulo),
            '-' => match self.peek() {
                '=' => self.pass(Special::SubAssign),
                '-' => self.pass(Special::Decr),
                _ => (1, Special::Sub),
            },
            '<' => match self.peek() {
                '<' => self.pass(Special::RShift),
                '=' => self.pass(Special::Leq),
                _ => (1, Special::LessOrLAngle),
            },
            '>' => match self.peek() {
                '>' => self.pass(Special::LShift),
                '=' => self.pass(Special::Geq),
                _ => (1, Special::GreaterOrRAngle),
            },
            '*' => match self.peek() {
                '*' => self.pass(Special::Power),
                '=' => self.pass(Special::MultAssign),
                _ => (1, Special::Mult),
            },
            '/' => match self.peek() {
                '/' => return None,
                '=' => self.pass(Special::DivAssign),
                _ => (1, Special::Div),
            },
            '+' => match self.peek() {
                '+' => self.pass(Special::Incr),
                '=' => self.pass(Special::AddAssign),
                _ => (1, Special::Add),
            },
            '!' => match self.peek() {
                '=' => self.pass(Special::Neq),
                _ => (1, Special::Exclamation),
            },
            _ => todo!(),
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
