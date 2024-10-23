use super::*;
use latex_token::{data::rose::TexRoseTokenData, idx::TexTokenIdx};

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TexRoseAstData {
    TextEdit { buffer: String },
}

impl<'a> TexAstParser<'a> {
    pub(super) fn parse_atomic_text_ast(
        &mut self,
        idx: TexTokenIdx,
        token: TexRoseTokenData,
    ) -> TexRoseAstData {
        match token {
            TexRoseTokenData::Word(_) => todo!(),
            TexRoseTokenData::Command(_) => todo!(),
            TexRoseTokenData::Dollar => todo!(),
            TexRoseTokenData::Nat32(_) => todo!(),
            TexRoseTokenData::NewParagraph => todo!(),
        }
    }
}
