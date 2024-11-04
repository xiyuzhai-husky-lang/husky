pub mod helpers;
#[cfg(test)]
mod tests;

use super::*;
use latex_command::{path::LxCommandPath, signature::parameter::LxCommandParameterMode};
use latex_token::{
    data::math::{digit::LxMathDigit, LxMathDelimiter},
    idx::{LxMathTokenIdx, LxTokenIdxRange},
};
use smallvec::{smallvec, SmallVec};

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LxMathAstData {
    Letter(LxMathTokenIdx, LxMathLetter),
    Punctuation(LxMathTokenIdx, LxMathPunctuation),
    Digit(LxMathTokenIdx, LxMathDigit),
    /// not obtained through parsing, but through ui
    TextEdit {
        buffer: String,
    },
    Attach {
        base: LxMathAstIdx,
        scripts: Vec<(LxScriptKind, LxMathAstIdx)>,
    },
    Delimited {
        left_delimiter_token_idx: LxMathTokenIdx,
        left_delimiter: LxMathDelimiter,
        asts: LxMathAstIdxRange,
        right_delimiter_token_idx: LxMathTokenIdx,
        right_delimiter: LxMathDelimiter,
    },
    Command {
        command_token_idx: LxMathTokenIdx,
        command_path: LxCommandPath,
        arguments: SmallVec<[LxMathCommandArgument; 2]>,
    },
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxMathCommandArgument {
    lcurl_token_idx: LxMathTokenIdx,
    data: LxMathCommandArgumentData,
    rcurl_token_idx: LxMathTokenIdx,
}

impl LxMathCommandArgument {
    pub fn lcurl_token_idx(&self) -> LxMathTokenIdx {
        self.lcurl_token_idx
    }

    pub fn data(&self) -> &LxMathCommandArgumentData {
        &self.data
    }

    pub fn rcurl_token_idx(&self) -> LxMathTokenIdx {
        self.rcurl_token_idx
    }

    pub fn asts_token_idx_range(&self) -> LxTokenIdxRange {
        ((*self.lcurl_token_idx + 1)..*self.rcurl_token_idx).into()
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LxMathCommandArgumentData {
    Math(LxMathAstIdxRange),
    Rose(LxRoseAstIdxRange),
    Letter(LxMathTokenIdx, LxMathLetter),
}

pub type LxMathAstArena = Arena<LxMathAstData>;
pub type LxMathAstArenaRef<'a> = ArenaRef<'a, LxMathAstData>;
pub type LxMathAstArenaMap<T> = ArenaMap<LxMathAstData, T>;
pub type LxMathAstIdx = ArenaIdx<LxMathAstData>;
pub type LxMathAstIdxRange = ArenaIdxRange<LxMathAstData>;

impl<'a> LxAstParser<'a> {
    pub(super) fn parse_atomic_math_ast(&mut self) -> Option<LxMathAstData> {
        match self.peek_math_token_data()? {
            LxMathTokenData::RightDelimiter(_) | LxMathTokenData::MathModeEnd => return None,
            _ => (),
        };
        let (idx, token) = self.next_math_token()?;
        Some(match token {
            LxMathTokenData::Command(command_name) => {
                let Ok(command_name) = command_name else {
                    todo!()
                };
                let command_signature = &self.command_signature_table()[command_name];
                let command_path = command_signature.path();
                let mut arguments: SmallVec<[LxMathCommandArgument; 2]> = smallvec![];
                for parameter in command_signature.parameters() {
                    arguments.push(self.parse_command_argument(parameter.mode())?);
                }
                LxMathAstData::Command {
                    command_token_idx: idx,
                    command_path,
                    arguments,
                }
            }
            LxMathTokenData::LeftDelimiter(delimiter) => self.parse_delimited(idx, delimiter),
            LxMathTokenData::RightDelimiter(_) => unreachable!(),
            LxMathTokenData::Letter(letter) => LxMathAstData::Letter(idx, letter),
            LxMathTokenData::Punctuation(opr) => LxMathAstData::Punctuation(idx, opr), // it's not constructed into a tree yet in the ast stage
            LxMathTokenData::Digit(digit) => LxMathAstData::Digit(idx, digit),
            LxMathTokenData::Other(_) => todo!(),
            LxMathTokenData::Subscript => todo!(),
            LxMathTokenData::Superscript => todo!(),
            LxMathTokenData::Error(_) => todo!(),
            LxMathTokenData::MathModeEnd => unreachable!(),
        })
    }

    fn parse_command_argument(
        &mut self,
        mode: LxCommandParameterMode,
    ) -> Option<LxMathCommandArgument> {
        match self.peek_math_token_data()? {
            LxMathTokenData::LeftDelimiter(LxMathDelimiter::Curl) => (),
            _ => return None,
        }
        let (lcurl_token_idx, LxMathTokenData::LeftDelimiter(LxMathDelimiter::Curl)) =
            self.next_math_token()?
        else {
            unreachable!()
        };
        let data = match mode {
            LxCommandParameterMode::Math => LxMathCommandArgumentData::Math(self.parse_math_asts()),
            LxCommandParameterMode::Rose => LxMathCommandArgumentData::Rose(self.parse_rose_asts()),
            LxCommandParameterMode::SingleLetter => todo!(),
        };
        let (rcurl_token_idx, LxMathTokenData::RightDelimiter(LxMathDelimiter::Curl)) =
            self.next_math_token()?
        else {
            todo!("report error properly")
        };
        Some(LxMathCommandArgument {
            lcurl_token_idx,
            data,
            rcurl_token_idx,
        })
    }

    // here we differ from the latex syntax, we see all possible delimiters as latex delimiters
    fn parse_delimited(
        &mut self,
        left_delimiter_token_idx: LxMathTokenIdx,
        left_delimiter: LxMathDelimiter,
    ) -> LxMathAstData {
        let asts = self.parse_math_asts();
        let Some((idx, token)) = self.next_math_token() else {
            todo!()
        };
        match token {
            LxMathTokenData::Command(_) => todo!(),
            LxMathTokenData::LeftDelimiter(_) => todo!(),
            LxMathTokenData::RightDelimiter(right_delimiter) => LxMathAstData::Delimited {
                left_delimiter_token_idx,
                left_delimiter,
                asts,
                right_delimiter_token_idx: idx,
                right_delimiter,
            },
            LxMathTokenData::Letter(_) => todo!(),
            LxMathTokenData::Punctuation(_) => todo!(),
            LxMathTokenData::Digit(_) => todo!(),
            LxMathTokenData::Other(_) => todo!(),
            LxMathTokenData::Subscript => todo!(),
            LxMathTokenData::Superscript => todo!(),
            LxMathTokenData::Error(_) => todo!(),
            LxMathTokenData::MathModeEnd => todo!(),
        }
    }
}
