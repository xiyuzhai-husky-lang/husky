pub mod helpers;
#[cfg(test)]
mod tests;

use super::*;
use latex_command::path::LxCommandPath;
use latex_token::{
    data::lisp::{LxLispDelimiter, LxLispIdent, LxLispTokenData},
    idx::LxLispTokenIdx,
};
use smallvec::{smallvec, SmallVec};

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LxLispAstData {
    Ident(LxLispTokenIdx, LxLispIdent),
    CompleteCommand {
        command_token_idx: LxLispTokenIdx,
        command_path: LxCommandPath,
        arguments: SmallVec<[LxLispCommandArgument; 2]>,
    },
    Parenthesized {
        lpar_token_idx: LxLispTokenIdx,
        asts: LxLispAstIdxRange,
        rpar_token_idx: LxLispTokenIdx,
    },
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxLispCommandArgument {
    lcurl_token_idx: LxLispTokenIdx,
    data: LxLispCommandArgumentData,
    rcurl_token_idx: LxLispTokenIdx,
}

pub type LxLispAstArena = Arena<LxLispAstData>;
pub type LxLispAstArenaRef<'a> = ArenaRef<'a, LxLispAstData>;
pub type LxLispAstArenaMap<T> = ArenaMap<LxLispAstData, T>;
pub type LxLispAstIdx = ArenaIdx<LxLispAstData>;
pub type LxLispAstIdxRange = ArenaIdxRange<LxLispAstData>;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LxLispCommandArgumentData {}

impl<'a> LxAstParser<'a> {
    pub(super) fn parse_lisp_asts(&mut self) -> LxLispAstIdxRange {
        let mut asts = vec![];
        while let Some(ast) = self.parse_lisp_ast() {
            asts.push(ast)
        }
        self.alloc_lisp_asts(asts)
    }

    fn parse_lisp_ast(&mut self) -> Option<LxLispAstData> {
        match self.peek_lisp_token_data()? {
            LxLispTokenData::Literal => (),
            LxLispTokenData::Ident(ident) => (),
            LxLispTokenData::LeftDelimiter(delimiter) => (),
            LxLispTokenData::RightDelimiter(delimiter) => return None,
            LxLispTokenData::Command(command_name) => (),
            LxLispTokenData::Comma => return None,
        }
        let (token_idx, token_data) = self.next_lisp_token()?;
        match token_data {
            LxLispTokenData::Literal => todo!(),
            LxLispTokenData::Ident(ident) => Some(LxLispAstData::Ident(token_idx, ident)),
            LxLispTokenData::LeftDelimiter(delimiter) => {
                self.parse_delimited_lisp_ast(token_idx, delimiter)
            }
            LxLispTokenData::RightDelimiter(delimiter) => todo!(),
            LxLispTokenData::Command(command_name) => todo!(),
            LxLispTokenData::Comma => todo!(),
        }
    }

    fn parse_delimited_lisp_ast(
        &mut self,
        token_idx: LxLispTokenIdx,
        delimiter: LxLispDelimiter,
    ) -> Option<LxLispAstData> {
        match delimiter {
            LxLispDelimiter::Parenthesis => self.parse_parenthesized_lisp_ast(token_idx),
            LxLispDelimiter::Box => todo!(),
        }
    }

    fn parse_parenthesized_lisp_ast(
        &mut self,
        lpar_token_idx: LxLispTokenIdx,
    ) -> Option<LxLispAstData> {
        let asts = self.parse_lisp_asts();
        let Some((rpar_token_idx, rpar_token_data)) = self.next_lisp_token() else {
            todo!()
        };
        match rpar_token_data {
            LxLispTokenData::RightDelimiter(LxLispDelimiter::Parenthesis) => (),
            _ => todo!(),
        }
        Some(LxLispAstData::Parenthesized {
            lpar_token_idx,
            asts,
            rpar_token_idx,
        })
    }

    fn parse_bracketed_lisp_ast(&mut self, token_idx: LxLispTokenIdx) -> Option<LxLispAstData> {
        todo!()
    }
}
