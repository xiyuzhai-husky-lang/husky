use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TexMathAstData {
    Letter(TexMathLetter),
    Opr(TexMathOpr),
    Nat32(u32),
    TextEdit { buffer: String },
}

impl<'a> TexAstParser<'a> {
    pub(super) fn parse_atomic_math_ast(&mut self, token: TexMathTokenData) -> TexMathAstData {
        match token {
            TexMathTokenData::Command(_) => todo!(),
            TexMathTokenData::LeftDelimiter(_) => todo!(),
            TexMathTokenData::RightDelimiter(_) => todo!(),
            TexMathTokenData::Letter(letter) => TexMathAstData::Letter(letter),
            TexMathTokenData::Opr(opr) => TexMathAstData::Opr(opr),
            TexMathTokenData::Nat32(number) => TexMathAstData::Nat32(number),
            TexMathTokenData::Other(_) => todo!(),
            TexMathTokenData::Subscript => todo!(),
            TexMathTokenData::Superscript => todo!(),
        }
    }
}
