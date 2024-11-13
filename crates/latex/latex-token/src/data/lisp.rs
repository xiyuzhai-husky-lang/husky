pub mod delimiter;
pub mod ident;
pub mod label;
pub mod literal;

use self::{delimiter::LxLispDelimiter, ident::LxLispIdent, literal::LxLispLiteral};
use super::*;
use husky_coword::Coword;
use label::LxLispXlabel;
use latex_command::path::LxCommandName;
use ordered_float::NotNan;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxLispTokenData {
    Literal(LxLispLiteral),
    Ident(LxLispIdent),
    Xlabel(LxLispXlabel),
    LeftDelimiter(LxLispDelimiter),
    RightDelimiter(LxLispDelimiter),
    Command(LxCommandName),
    Comma,
}

impl<'a> LxLexer<'a> {
    pub(crate) fn next_lisp_token_data(&mut self) -> Option<LxLispTokenData> {
        let db = self.db;
        Some(match self.chars.peek()? {
            '}' => return None,
            '\\' => {
                self.chars.eat_char();
                match self.chars.peek() {
                    Some(c) => match c {
                        c if c.is_alphabetic() => {
                            let Ok(command_name) = LxCommandName::new2(
                                self.chars.next_str_slice_while(|c| c.is_alphabetic()),
                                db,
                            ) else {
                                todo!()
                            };
                            LxLispTokenData::Command(command_name)
                        }
                        c => {
                            todo!()
                        }
                    },
                    None => todo!(),
                }
            }
            n if n.is_numeric() => {
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
                        LxLispLiteral::Int(i)
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
                        LxLispLiteral::Float(f)
                    }
                    _ => unreachable!(),
                };
                LxLispTokenData::Literal(literal)
            }
            c if c.is_alphabetic() => {
                let ident = LxLispIdent::new(
                    self.chars
                        .next_str_slice_while(|c| c.is_alphanumeric() || c == '_'),
                    db,
                );
                LxLispTokenData::Ident(ident)
            }
            '\'' => {
                self.chars.eat_char();
                let label = LxLispXlabel::new(
                    self.chars
                        .next_str_slice_while(|c| c.is_alphanumeric() || c == '-' || c == ':'),
                    db,
                );
                LxLispTokenData::Xlabel(label)
            }
            c => {
                self.chars.eat_char();
                match c {
                    '(' => LxLispTokenData::LeftDelimiter(LxLispDelimiter::Parenthesis),
                    ')' => LxLispTokenData::RightDelimiter(LxLispDelimiter::Parenthesis),
                    '[' => LxLispTokenData::LeftDelimiter(LxLispDelimiter::Box),
                    ']' => LxLispTokenData::RightDelimiter(LxLispDelimiter::Box),
                    ',' => LxLispTokenData::Comma,
                    c => todo!("c: {}", c),
                }
            }
        })
    }
}
