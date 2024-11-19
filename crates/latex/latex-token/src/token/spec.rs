mod literal;

use self::literal::LxSpecLiteral;
use super::*;
use husky_coword::Coword;
use latex_command::path::LxCommandName;
use ordered_float::NotNan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LxSpecTokenData {
    Command(LxCommandName),
    RightDelimiter(LxSpecDelimiter),
    Coword(Coword),
    Literal(LxSpecLiteral),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LxSpecDelimiter {
    Curl,
    Box,
}

impl<'a> LxLexer<'a> {
    pub(crate) fn next_spec_token_data(&mut self) -> Option<LxSpecTokenData> {
        let db = self.db;
        match self.chars.peek()? {
            '\\' => {
                self.chars.eat_char();
                match self.chars.peek() {
                    Some(c) => match c {
                        c if c.is_ascii_alphabetic() => Some(LxSpecTokenData::Command(
                            LxCommandName::new(
                                self.next_coword_with(|c| c.is_ascii_alphabetic()).unwrap(),
                                db,
                            )
                            .unwrap(),
                        )),
                        c if c.is_numeric() => todo!("latex might allow single digit command"),
                        _ => todo!("latex one digit non letter command"),
                    },
                    None => todo!(),
                }
            }
            c if c.is_ascii_alphabetic() => Some(LxSpecTokenData::Coword(
                self.next_coword_with(|c| c.is_ascii_alphanumeric())
                    .unwrap(),
            )),
            n if n.is_ascii_digit() => {
                let mut dot_count = 0;
                let s = self.chars.next_str_slice_while(|c| {
                    if c == '.' {
                        dot_count += 1;
                    }
                    (c.is_numeric() || c == '.') && dot_count < 2
                });
                let literal = match dot_count {
                    0 => {
                        let i = match s.parse() {
                            Ok(i) => i,
                            Err(e) => todo!("{}", e),
                        };
                        LxSpecLiteral::Int(i)
                    }
                    1 => {
                        let f = match s.parse::<f64>() {
                            Ok(f) => f,
                            Err(e) => todo!("{}", e),
                        };
                        let f = match NotNan::new(f) {
                            Ok(f) => f,
                            Err(e) => todo!("{}", e),
                        };
                        LxSpecLiteral::Float(f)
                    }
                    _ => unreachable!(),
                };
                Some(LxSpecTokenData::Literal(literal))
            }
            ']' => {
                self.chars.eat_char();
                Some(LxSpecTokenData::RightDelimiter(LxSpecDelimiter::Box))
            }
            c => todo!("c: {:?}", c),
        }
    }
}
