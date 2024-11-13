use super::*;
use husky_coword::Coword;
use latex_command::path::LxCommandName;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxLispTokenData {
    Literal,
    Ident(LxLispIdent),
    LeftDelimiter(LxLispDelimiter),
    RightDelimiter(LxLispDelimiter),
    Command(LxCommandName),
    Comma,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LxLispIdent(Coword);

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxLispDelimiter {
    Parenthesis,
    Box,
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
                todo!()
            }
            c if c.is_alphabetic() => {
                let ident = LxLispIdent(Coword::from_ref(
                    db,
                    self.chars.next_str_slice_while(|c| c.is_alphabetic()),
                ));
                LxLispTokenData::Ident(ident)
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
