//! means the prose mode
#[cfg(test)]
pub mod tests;

use super::*;
use latex_token::{data::rose::LxRoseTokenData, idx::LxRoseTokenIdx};

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LxRoseAstData {
    TextEdit { buffer: String },
}

pub type LxRoseAstArena = Arena<LxRoseAstData>;
pub type LxRoseAstArenaRef<'a> = ArenaRef<'a, LxRoseAstData>;
pub type LxRoseAstArenaMap<T> = ArenaMap<LxRoseAstData, T>;
pub type LxRoseAstIdx = ArenaIdx<LxRoseAstData>;
pub type LxRoseAstIdxRange = ArenaIdxRange<LxRoseAstData>;

impl<'a> LxAstParser<'a> {
    pub(super) fn parse_atomic_text_ast(
        &mut self,
        idx: LxRoseTokenIdx,
        token: LxRoseTokenData,
    ) -> LxRoseAstData {
        match token {
            LxRoseTokenData::Word(_) => todo!(),
            LxRoseTokenData::Command(_) => todo!(),
            LxRoseTokenData::Dollar => todo!(),
            LxRoseTokenData::EscapedLpar => todo!(),
            LxRoseTokenData::EscapedLbox => todo!(),
            LxRoseTokenData::Nat32(_) => todo!(),
            LxRoseTokenData::NewParagraph => todo!(),
        }
    }
}
