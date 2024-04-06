use super::*;
use husky_tex_token::{data::text::TexTextTokenData, idx::TexTokenIdx};

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TexTextAstData {
    TextEdit { buffer: String },
}

impl<'a> TexAstParser<'a> {
    pub(super) fn parse_atomic_text_ast(
        &mut self,
        idx: TexTokenIdx,
        token: TexTextTokenData,
    ) -> TexTextAstData {
        match token {
            TexTextTokenData::Word(_) => todo!(),
            TexTextTokenData::Command(_) => todo!(),
            TexTextTokenData::Dollar => todo!(),
            TexTextTokenData::Nat32(_) => todo!(),
        }
    }
}
