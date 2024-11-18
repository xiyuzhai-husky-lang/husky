use super::*;
use husky_coword::Coword;
use latex_command::path::LxCommandName;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LxSpecTokenData {
    Command(LxCommandName),
    RightDelimiter(LxSpecDelimiter),
    Coword(Coword),
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
            ']' => {
                self.chars.eat_char();
                Some(LxSpecTokenData::RightDelimiter(LxSpecDelimiter::Box))
            }
            _ => todo!(),
        }
    }
}
