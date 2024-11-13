use super::*;
use latex_command::path::LxCommandPath;
use latex_token::idx::LxLispTokenIdx;
use smallvec::{smallvec, SmallVec};

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LxLispAstData {
    CompleteCommand {
        command_token_idx: LxLispTokenIdx,
        command_path: LxCommandPath,
        arguments: SmallVec<[LxLispCommandArgument; 2]>,
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
        todo!()
    }
}
