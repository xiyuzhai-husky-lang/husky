//! means the prose mode
use super::*;
use latex_token::{data::rose::LxRoseTokenData, idx::LxTokenIdx};

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LxRoseAstData {
    TextEdit { buffer: String },
}

impl<'a> LxAstParser<'a> {
    pub(super) fn parse_atomic_text_ast(
        &mut self,
        idx: LxTokenIdx,
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
